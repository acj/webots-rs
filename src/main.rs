use webots_rs::bindings::WbDeviceTag;

fn main() {
    const INFINITY: f64 = 1.0 / 0.0;
    const MAX_SPEED: f64 = 6.28;
    const TIME_STEP: i32 = 64;

    println!("Rust controller has started");
    webots_rs::wb_robot_init();

    let distance_sensor_names = vec!["ps0", "ps1", "ps2", "ps3", "ps4", "ps5", "ps6", "ps7"];
    let distance_sensors: Vec<WbDeviceTag> = distance_sensor_names
        .iter()
        .map(|name| {
            let sensor: WbDeviceTag = webots_rs::wb_robot_get_device(name);
            webots_rs::wb_distance_sensor_enable(sensor, TIME_STEP);
            sensor
        })
        .collect();

    let left_motor = webots_rs::wb_robot_get_device("left wheel motor");
    let right_motor = webots_rs::wb_robot_get_device("right wheel motor");
    webots_rs::wb_motor_set_position(left_motor, INFINITY);
    webots_rs::wb_motor_set_position(right_motor, INFINITY);

    webots_rs::wb_motor_set_velocity(left_motor, 0.1 * MAX_SPEED);
    webots_rs::wb_motor_set_velocity(right_motor, 0.1 * MAX_SPEED);

    loop {
        if webots_rs::wb_robot_step(TIME_STEP) == -1 {
            break;
        }

        let distance_values: Vec<f64> = distance_sensors
            .iter()
            .map(|sensor| webots_rs::wb_distance_sensor_get_value(*sensor))
            .collect();

        // detect obsctacles
        let left_obstacle =
            distance_values[5] > 80.0 || distance_values[6] > 80.0 || distance_values[7] > 80.0;
        let right_obstacle =
            distance_values[0] > 80.0 || distance_values[1] > 80.0 || distance_values[2] > 80.0;

        // initialize motor speeds at 50% of MAX_SPEED.
        let mut left_speed = 0.5 * MAX_SPEED;
        let mut right_speed = 0.5 * MAX_SPEED;

        // modify speeds according to obstacles
        if left_obstacle {
            // turn right
            left_speed += 0.5 * MAX_SPEED;
            right_speed -= 0.5 * MAX_SPEED;
        } else if right_obstacle {
            // turn left
            left_speed -= 0.5 * MAX_SPEED;
            right_speed += 0.5 * MAX_SPEED;
        }

        // write actuators inputs
        webots_rs::wb_motor_set_velocity(left_motor, left_speed);
        webots_rs::wb_motor_set_velocity(right_motor, right_speed);
    }

    webots_rs::wb_robot_cleanup();
}
