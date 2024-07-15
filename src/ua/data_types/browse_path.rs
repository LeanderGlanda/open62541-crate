use crate::data_type::DataType;
use crate::ua;
use crate::ua::NodeId;

crate::data_type!(BrowsePath);

impl BrowsePath {
    #[must_use]
    pub fn with_starting_node(mut self, node_id: &NodeId) -> Self {
        node_id.clone_into_raw(&mut self.0.startingNode);
        self
    }

    #[must_use]
    pub fn with_relative_path_element_size(mut self, element_size: usize) -> Self {
        self.0.relativePath.elementsSize = element_size;
        self
    }

    #[must_use]
    pub fn with_relative_path_elements(mut self, elements: ua::RelativePathElement) -> Self {
        // SAFETY: Pass ownership to self so the RelativePathElement will be freed when self will be freed
        // otherwise the RelativePathElement would have already been freed by the time self would be.
        self.0.relativePath.elements = elements.leak_into_raw();
        self
    }
}
