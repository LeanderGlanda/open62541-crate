use crate::data_type::DataType;
use crate::ua;

crate::data_type!(BrowsePathTarget);

impl BrowsePathTarget {
    #[must_use]
    pub fn get_target_id(&self) -> ua::ExpandedNodeId {
        ua::ExpandedNodeId::clone_raw(&self.0.targetId)
    }
}
