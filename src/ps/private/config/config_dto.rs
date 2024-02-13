use super::super::utils;
use super::branch::BranchConfigDto;
use super::fetch::FetchConfigDto;
use super::integrate::IntegrateConfigDto;
use super::list::ListConfigDto;
use super::pull::PullConfigDto;
use super::request_review::RequestReviewConfigDto;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct ConfigDto {
    pub request_review: Option<RequestReviewConfigDto>,
    pub pull: Option<PullConfigDto>,
    pub integrate: Option<IntegrateConfigDto>,
    pub fetch: Option<FetchConfigDto>,
    pub list: Option<ListConfigDto>,
    pub branch: Option<BranchConfigDto>,
}

impl utils::Mergable for ConfigDto {
    fn merge(&self, b: &Self) -> Self {
        ConfigDto {
            request_review: utils::merge_option(&self.request_review, &b.request_review),
            pull: utils::merge_option(&self.pull, &b.pull),
            integrate: utils::merge_option(&self.integrate, &b.integrate),
            fetch: utils::merge_option(&self.fetch, &b.fetch),
            list: utils::merge_option(&self.list, &b.list),
            branch: utils::merge_option(&self.branch, &b.branch),
        }
    }
}
