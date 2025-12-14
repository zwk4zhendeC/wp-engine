use orion_conf::{ErrorOwe, ErrorWith};
use std::fs;
use std::io::Write;
use std::path::Path;
use wp_error::run_error::RunResult;

macro_rules! connector_template {
    ($rel:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../connectors/",
            $rel
        ))
    };
}

fn write_templates_if_absent(dir: &Path, templates: &[(&str, &str)]) -> RunResult<()> {
    fs::create_dir_all(dir)
        .owe_res()
        .want("create dir")
        .with(dir)?;
    for (name, body) in templates {
        let p = dir.join(name);
        if !p.exists() {
            let mut f = fs::File::create(&p)
                .owe_res()
                .want("create file")
                .with(&p)?;
            f.write_all(body.as_bytes())
                .owe_res()
                .want("write file")
                .with(&p)?;
        }
    }
    Ok(())
}

fn ensure_source_connectors<P: AsRef<std::path::Path>>(work_root: P) -> RunResult<()> {
    let dir = work_root.as_ref().join("connectors").join("source.d");
    let templates: &[(&str, &str)] = &[
        (
            "00-file-default.toml",
            connector_template!("source.d/00-file-default.toml"),
        ),
        (
            "10-syslog-udp.toml",
            connector_template!("source.d/10-syslog-udp.toml"),
        ),
        (
            "11-syslog-tcp.toml",
            connector_template!("source.d/11-syslog-tcp.toml"),
        ),
        (
            "30-kafka.toml",
            connector_template!("source.d/30-kafka.toml"),
        ),
    ];
    write_templates_if_absent(&dir, templates)
}

fn ensure_sink_connectors<P: AsRef<std::path::Path>>(work_root: P) -> RunResult<()> {
    let dir = work_root.as_ref().join("connectors").join("sink.d");
    let templates: &[(&str, &str)] = &[
        (
            "01-file-prototext.toml",
            connector_template!("sink.d/01-file-prototext.toml"),
        ),
        (
            "02-file-json.toml",
            connector_template!("sink.d/02-file-json.toml"),
        ),
        (
            "03-file-kv.toml",
            connector_template!("sink.d/03-file-kv.toml"),
        ),
        (
            "04-file-raw.toml",
            connector_template!("sink.d/04-file-raw.toml"),
        ),
        (
            "10-syslog-udp.toml",
            connector_template!("sink.d/10-syslog-udp.toml"),
        ),
        (
            "11-syslog-tcp.toml",
            connector_template!("sink.d/11-syslog-tcp.toml"),
        ),
        ("30-kafka.toml", connector_template!("sink.d/30-kafka.toml")),
        (
            "30-prometheus.toml",
            connector_template!("sink.d/40-prometheus.toml"),
        ),
    ];
    write_templates_if_absent(&dir, templates)
}

pub fn init_templates<P: AsRef<Path>>(work_root: P) -> RunResult<()> {
    ensure_source_connectors(&work_root)?;
    ensure_sink_connectors(&work_root)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::temp_workdir;

    #[test]
    fn init_templates_creates_expected_files() {
        let temp = temp_workdir();
        init_templates(temp.path().to_str().unwrap()).expect("init templates");

        let source_file = temp.path().join("connectors/source.d/00-file-default.toml");
        let sink_file = temp.path().join("connectors/sink.d/02-file-json.toml");
        assert!(source_file.exists());
        assert!(sink_file.exists());
    }

    #[test]
    fn init_templates_does_not_overwrite_existing_file() {
        let temp = temp_workdir();
        let custom = temp.path().join("connectors/source.d/00-file-default.toml");
        std::fs::create_dir_all(custom.parent().unwrap()).unwrap();
        std::fs::write(&custom, "[[connectors]]\ncustom = true\n").unwrap();

        init_templates(temp.path().to_str().unwrap()).expect("init templates");

        let body = std::fs::read_to_string(&custom).unwrap();
        assert!(body.contains("custom"));
    }
}
