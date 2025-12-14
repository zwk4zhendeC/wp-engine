//! Workshop 解析引擎模块

mod ximpl;

pub mod batch_processor;
pub mod data_sender;
pub mod packet_parser;
pub mod types;

use super::super::prelude::*;
use super::WplRepository;
use crate::core::parser::indexing::ResourceIndexer;
use crate::core::parser::wpl_engine::pipeline::WplPipeline;
use crate::resources::sinks::null_resources::AssignRes;
use crate::resources::utils::code_ins_parse_units;
use crate::sinks::{InfraSinkAgent, SinkGroupAgent, SinkInfraAble};

#[derive(Getters)]
pub struct WplWorkshop {
    pub pipelines: Vec<WplPipeline>,
    pub conveyor: InfraSinkAgent,
}

impl WplWorkshop {
    pub(crate) fn from(pipe_lines: Vec<WplPipeline>, infra: InfraSinkAgent) -> AnyResult<Self> {
        // 规则命中排序
        let mut pipelines = pipe_lines;
        pipelines.sort_by(|a, b| b.hit_cnt.cmp(&a.hit_cnt));

        Ok(Self {
            pipelines,
            conveyor: infra,
        })
    }

    pub fn from_code(wpl_spc: &WplRepository, infra: InfraSinkAgent) -> AnyResult<Self> {
        let mut idx = ResourceIndexer::default();
        let mut pipelines = Vec::new();
        for pkg in &wpl_spc.packages {
            let mut units =
                code_ins_parse_units(AssignRes::use_it(infra.default().clone()), pkg, &mut idx)?;
            pipelines.append(&mut units);
        }
        Self::from(pipelines, infra)
    }

    pub fn stop(&mut self) {
        self.pipelines.iter_mut().for_each(|i| i.stop());
    }

    pub fn optimized(&mut self, _count: usize) {
        if self.pipelines.is_empty() {
            return;
        }

        self.pipelines.sort_by(|a, b| {
            b.hit_cnt
                .cmp(&a.hit_cnt)
                .then_with(|| a.index().cmp(b.index()))
        });

        // 下一窗口重新记录命中情况
        for pipeline in &mut self.pipelines {
            pipeline.hit_cnt = 0;
        }
    }

    /// 更新规则命中计数并排序
    pub fn hit_count_sort(&mut self) {
        for x in &mut self.pipelines {
            x.access_cnt = 0;
        }
    }
}

impl SinkInfraAble for WplWorkshop {
    fn miss(&self) -> SinkGroupAgent {
        self.conveyor.miss.clone()
    }

    fn error(&self) -> SinkGroupAgent {
        self.conveyor.error.clone()
    }

    fn residue(&self) -> SinkGroupAgent {
        self.conveyor.residue.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::parser::wpl_engine::engine::ParsingEngine;
    use crate::core::parser::wpl_engine::pipeline::WplPipeline;
    use crate::sinks::{InfraSinkAgent, SinkGroupAgent};
    use wpl::WplEvaluator;

    fn dummy_pipeline(idx: usize, hit: usize) -> WplPipeline {
        let evaluator = WplEvaluator::from_code("rule dummy { ( _ ) }").expect("build wpl");
        let parser = ParsingEngine::RuleEngine(evaluator);
        let mut pipeline = WplPipeline::new(
            idx,
            format!("rule-{}", idx),
            Vec::new(),
            parser,
            vec![SinkGroupAgent::null()],
            Vec::new(),
        );
        pipeline.hit_cnt = hit;
        pipeline
    }

    #[test]
    fn optimized_reorders_by_hit_count() {
        let infra = InfraSinkAgent::use_null();
        let pipelines = vec![
            dummy_pipeline(0, 1),
            dummy_pipeline(1, 5),
            dummy_pipeline(2, 3),
        ];

        let mut workshop = WplWorkshop::from(pipelines, infra).expect("build workshop");
        workshop.optimized(0);

        let order: Vec<_> = workshop
            .pipelines()
            .iter()
            .map(|p| p.wpl_key().to_string())
            .collect();
        assert_eq!(order, vec!["rule-1", "rule-2", "rule-0"]);
        assert!(workshop.pipelines().iter().all(|p| p.hit_cnt == 0));
    }
}

// 重新导出主要类型
