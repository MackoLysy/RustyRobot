#[path = "point.rs"]
mod point;

use point::Point;
use serde_json::{Result, Value};
pub struct RustyBot {
    err: i32,
    err_msg: String,
    points: Vec<Point>,
    x: i64,
    y: i64,
}

impl RustyBot {
    pub fn new() -> RustyBot {
        RustyBot {
            err: 0,
            err_msg: "OK".to_string(),
            points: Vec::new(),
            x: 0,
            y: 0,
        }
    }

    pub fn get_message(&mut self, message: std::string::String) {
        let val: Result<Value> = serde_json::from_str(message.as_str());
        if val.is_ok() {
            let json = val.unwrap();
            self.parse_json(json);
        } else {
            println!("Not a json!");
            self.err = 1;
            self.err_msg = "error in JSON parse".to_string();
        }
    }

    fn parse_json(&mut self, json: Value) {
        let cmd = json["cmd"].as_str();
        if cmd.is_some() {
            match cmd.unwrap() {
                "distance" => {
                    let angle = json["angle"].as_i64();
                    let distance = json["distance"].as_i64();
                    if angle.is_some() && distance.is_some() {
                        self.add_point(distance.unwrap(), angle.unwrap());
                    }
                }
                _ => {
                    self.err = 2;
                    self.err_msg = "invalid cmd".to_string();
                }
            }
        } else {
            self.err = 2;
            self.err_msg = "invalid cmd".to_string();
        }
    }

    fn add_point(&mut self, dist: i64, angle: i64) {
        let mut point = Point::new(self.x, self.y);
        point.caluclate_point(dist, angle);
        self.points.push(point);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn get_message_valid() {
        let mut dummy = RustyBot::new();
        dummy.get_message("{\"test\":1}".to_string());
        assert_eq!(dummy.err, 0);
    }

    #[test]
    fn get_message_invalid() {
        let mut dummy = RustyBot::new();
        dummy.get_message("assds".to_string());
        assert_eq!(dummy.err, 1);
    }
    #[test]
    fn parse_message() {
        let mut dummy = RustyBot::new();
        let json = json!({
            "cmd": "distance"
        });
        dummy.parse_json(json);
        assert_eq!(dummy.err, 0);
    }

    #[test]
    fn parse_message_invalid() {
        let mut dummy = RustyBot::new();
        let json = json!({
            "cmd": "huehue"
        });
        dummy.parse_json(json);
        assert_eq!(dummy.err, 2);
    }
    #[test]
    fn push_point() {
        let mut dummy = RustyBot::new();
        dummy.add_point(10, 10);
        assert_eq!(dummy.points.len(), 1);
    }
    #[test]
    fn push_point_extend() {
        let mut dummy = RustyBot::new();
        let json = json!({
            "cmd": "distance",
            "distance": 10,
            "angle": 10
        });
        dummy.parse_json(json);
        assert_eq!(dummy.points.len(), 1);
    }
}
