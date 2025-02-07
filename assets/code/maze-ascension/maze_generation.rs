/// Spawns a new maze floor in response to a SpawnMaze trigger event
pub(super) fn spawn_maze(
    trigger: Trigger<SpawnMaze>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    maze_query: Query<(Entity, &Floor, &Maze)>,
    global_config: Res<GlobalMazeConfig>,
) {
    let SpawnMaze { floor, config } = trigger.event();

    if maze_query.iter().any(|(_, f, _)| f.0 == *floor) {
        warn!("Floor {} already exists, skipping creation", floor);
        return;
    }

    let maze = match generate_maze(config) {
        Ok(m) => m,
        Err(e) => {
            error!("Failed to generate maze for floor {floor}: {:?}", e);
            return;
        }
    };

    // Calculate vertical offset based on floor number
    let y_offset = match *floor {
        1 => 0,              // Ground/Initial floor (floor 1) is at y=0
        _ => FLOOR_Y_OFFSET, // Other floors are offset vertically
    } as f32;

    let entity = commands
        .spawn((
            Name::new(format!("Floor {}", floor)),
            HexMaze,
            maze.clone(),
            Floor(*floor),
            config.clone(),
            Transform::from_translation(Vec3::ZERO.with_y(y_offset)),
            Visibility::Visible,
        ))
        .insert_if(CurrentFloor, || *floor == 1) // Only floor 1 gets CurrentFloor
        .id();

    let assets = MazeAssets::new(&mut meshes, &mut materials, &global_config);
    spawn_maze_tiles(
        &mut commands,
        entity,
        &maze,
        &assets,
        config,
        &global_config,
    );
}

/// Spawns all tiles for a maze as children of the parent maze entity
pub fn spawn_maze_tiles(
    commands: &mut Commands,
    parent_entity: Entity,
    maze: &Maze,
    assets: &MazeAssets,
    maze_config: &MazeConfig,
    global_config: &GlobalMazeConfig,
) {
    commands.entity(parent_entity).with_children(|parent| {
        for tile in maze.values() {
            spawn_single_hex_tile(
                parent,
                assets,
                tile,
                maze_config,
                global_config,
            );
        }
    });
}

/// Spawns a single hexagonal tile with appropriate transforms and materials
pub(super) fn spawn_single_hex_tile(
    parent: &mut ChildBuilder,
    assets: &MazeAssets,
    tile: &HexTile,
    maze_config: &MazeConfig,
    global_config: &GlobalMazeConfig,
) {
    let world_pos = tile.to_vec3(&maze_config.layout);
    let rotation = match maze_config.layout.orientation {
        // Pointy hexagons don't need additional rotation (0 degrees)
        HexOrientation::Pointy => Quat::from_rotation_y(0.0),
        // Flat-top hexagons need 30 degrees (pi/6) rotation around Y axis
        HexOrientation::Flat => Quat::from_rotation_y(FRAC_PI_6),
    };

    // Select material based on tile position: start, end, or default
    let material = match tile.pos() {
        pos if pos == maze_config.start_pos => assets
            .custom_materials
            .get(&RosePine::Pine)
            .cloned()
            .unwrap_or_default(),
        pos if pos == maze_config.end_pos => assets
            .custom_materials
            .get(&RosePine::Love)
            .cloned()
            .unwrap_or_default(),
        _ => assets.hex_material.clone(),
    };

    parent
        .spawn((
            Name::new(format!("Hex {}", tile)),
            Tile,
            Mesh3d(assets.hex_mesh.clone()),
            MeshMaterial3d(material),
            Transform::from_translation(world_pos).with_rotation(rotation),
        ))
        .with_children(|parent| {
            spawn_walls(parent, assets, tile.walls(), global_config)
        });
}

/// Spawns walls around a hexagonal tile based on the walls configuration
fn spawn_walls(
    parent: &mut ChildBuilder,
    assets: &MazeAssets,
    walls: &Walls,
    global_config: &GlobalMazeConfig,
) {
    // Base rotation for wall alignment (90 degrees counter-clockwise)
    let z_rotation = Quat::from_rotation_z(-FRAC_PI_2);
    let y_offset = global_config.height / 2.;

    for i in 0..6 {
        if !walls.contains(i) {
            continue;
        }

        // Calculate the angle for this wall
        // FRAC_PI_3 = 60 deg
        // Negative because going clockwise
        // i * 60 produces: 0, 60, 120, 180, 240, 300
        let wall_angle = -FRAC_PI_3 * i as f32;

        // cos(angle) gives x coordinate on unit circle
        // sin(angle) gives z coordinate on unit circle
        // Multiply by wall_offset to get actual distance from center
        let x_offset = global_config.wall_offset() * f32::cos(wall_angle);
        let z_offset = global_config.wall_offset() * f32::sin(wall_angle);

        // x: distance along x-axis from center
        // y: vertical offset from center
        // z: distance along z-axis from center
        let pos = Vec3::new(x_offset, y_offset, z_offset);

        // 1. Rotate around x-axis to align wall with angle
        // 2. Add FRAC_PI_2 (90) to make wall perpendicular to angle
        let x_rotation = Quat::from_rotation_x(wall_angle + FRAC_PI_2);
        let final_rotation = z_rotation * x_rotation;

        spawn_single_wall(parent, assets, final_rotation, pos);
    }
}

/// Spawns a single wall segment with the specified rotation and position
fn spawn_single_wall(
    parent: &mut ChildBuilder,
    assets: &MazeAssets,
    rotation: Quat,
    offset: Vec3,
) {
    parent.spawn((
        Name::new("Wall"),
        Wall,
        Mesh3d(assets.wall_mesh.clone()),
        MeshMaterial3d(assets.wall_material.clone()),
        Transform::from_translation(offset).with_rotation(rotation),
    ));
}
