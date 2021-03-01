use config::Config;
use config::Environment;
use config::File;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub enum MetricsExporter {
    TCP {
        socket_address: String,
        buffer_size: usize,
    },
    StatsD {
        host: String,
        port: u16,
        queue_size: usize,
        buffer_size: usize,
        prefix: String,
    },
}

fn main() {
    let environ = Environment::with_prefix("APP").separator("__");
    let expected = MetricsExporter::StatsD {
        host: String::from("127.0.0.1"),
        port: 8125,
        queue_size: 1,
        buffer_size: 1024,
        prefix: String::from("blah"),
    };
    let mut cfg = Config::default();
    cfg.merge(File::with_name("development.toml")).unwrap();
    cfg.merge(environ).unwrap();
    let m: MetricsExporter = cfg.get("metrics.exporter").unwrap();
    assert_eq!(expected, m);
}
