#[derive(Clone, Debug)]
pub struct GenGRA {
    pub total_line: Option<usize>,
    pub gen_speed: usize,
    pub parallel: usize,
    pub stat_sec: usize,
    pub stat_print: bool,
    pub rescue: String,
}

impl Default for GenGRA {
    fn default() -> Self {
        Self {
            total_line: Some(1000),
            gen_speed: 1000,
            parallel: 1,
            stat_sec: 1,
            stat_print: false,
            rescue: "./rescue".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SampleGRA {
    pub gen_conf: GenGRA,
}

#[derive(Clone, Debug, Default)]
pub struct RuleGRA {
    pub gen_conf: GenGRA,
}

impl GenGRA {
    /// 获取生成速率；若字段缺省返回默认值（与 Default 对齐）。
    /// 说明：用于在生成器直连路径上决定是否开启 backoff gate（gen_speed==0 视为无限速）。
    pub fn gen_conf_or_default_speed(&self) -> usize {
        self.gen_speed
    }
}
