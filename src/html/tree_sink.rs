use forest_ds::id::NodeId;
use html5ever::{
    expanded_name, local_name, namespace_url, ns,
    tendril::StrTendril,
    tree_builder::{ElementFlags, NodeOrText, QuirksMode, TreeSink},
    Attribute, ExpandedName, QualName,
};

use super::{
    node::{Comment, Doctype, Element, Node, ProcessingInstruction, Text},
    Html,
};

impl TreeSink for Html {
    type Handle = NodeId;

    type Output = Self;

    fn finish(self) -> Self::Output {
        self
    }

    fn parse_error(&mut self, msg: std::borrow::Cow<'static, str>) {
        self.errors.push(msg)
    }

    fn get_document(&mut self) -> Self::Handle {
        self.tree.first_node_id().unwrap()
    }

    fn elem_name<'a>(&'a self, target: &'a Self::Handle) -> ExpandedName<'a> {
        self.tree
            .get(target)
            .unwrap()
            .as_element()
            .unwrap()
            .name
            .expanded()
    }

    fn create_element(
        &mut self,
        name: QualName,
        attrs: Vec<Attribute>,
        _flags: ElementFlags,
    ) -> Self::Handle {
        let node = self
            .tree
            .create_node(Node::Element(Element::new(name.clone(), attrs)));

        if name.expanded() == expanded_name!(html "template") {
            let _ = self.tree.append_child_to(&node, Node::Fragment);
        }

        node
    }

    fn create_comment(&mut self, text: StrTendril) -> Self::Handle {
        self.tree
            .create_node(Node::Comment(Comment { comment: text }))
    }

    fn create_pi(&mut self, target: StrTendril, data: StrTendril) -> Self::Handle {
        self.tree
            .create_node(Node::ProcessingInstruction(ProcessingInstruction {
                target,
                data,
            }))
    }

    fn append(&mut self, parent: &Self::Handle, child: NodeOrText<Self::Handle>) {
        match child {
            NodeOrText::AppendNode(node) => self.tree.make_child(&node, parent).unwrap(),
            NodeOrText::AppendText(text) => {
                let mut cursor = self.tree.cursor(parent).unwrap();

                if cursor.last_child().is_ok() {
                    match cursor.get_mut() {
                        Node::Text(last_child) => last_child.text.push_tendril(&text),
                        _ => {}
                    }
                } else {
                    self.tree
                        .append_child_to(parent, Node::Text(Text { text }))
                        .unwrap();
                }
            }
        }
    }

    fn append_based_on_parent_node(
        &mut self,
        element: &Self::Handle,
        prev_element: &Self::Handle,
        child: NodeOrText<Self::Handle>,
    ) {
        let cursor = self.tree.cursor(element).unwrap();

        if cursor.peek_parent().is_some() {
            self.append(element, child)
        } else {
            self.append(prev_element, child)
        }
    }

    fn append_doctype_to_document(
        &mut self,
        name: StrTendril,
        public_id: StrTendril,
        system_id: StrTendril,
    ) {
        let doctype = Doctype {
            name,
            public_id,
            system_id,
        };

        let document = self.get_document();

        self.tree
            .append_child_to(&document, Node::Doctype(doctype))
            .unwrap();
    }

    fn get_template_contents(&mut self, target: &Self::Handle) -> Self::Handle {
        self.tree
            .cursor(target)
            .unwrap()
            .first_child()
            .unwrap()
            .id()
    }

    fn same_node(&self, x: &Self::Handle, y: &Self::Handle) -> bool {
        x == y
    }

    fn set_quirks_mode(&mut self, mode: QuirksMode) {
        self.quirks_mode = mode;
    }

    fn append_before_sibling(
        &mut self,
        sibling: &Self::Handle,
        new_node: NodeOrText<Self::Handle>,
    ) {
        match new_node {
            NodeOrText::AppendNode(node) => self.tree.make_prev_siblings(&node, sibling).unwrap(),
            NodeOrText::AppendText(text) => {
                let mut cursor = self.tree.cursor(sibling).unwrap();
                if cursor.prev_sibling().is_ok() {
                    if let Node::Text(prev_sibling) = cursor.get_mut() {
                        prev_sibling.text.push_tendril(&text);

                        return;
                    }
                }

                // Default
                let id = self.tree.create_node(Node::Text(Text { text }));
                self.tree.make_prev_siblings(&id, sibling).unwrap();
            }
        }
    }

    fn add_attrs_if_missing(&mut self, target: &Self::Handle, attributes: Vec<Attribute>) {
        match self.tree.get_mut(target).unwrap() {
            Node::Element(element) => attributes.into_iter().for_each(|attr| {
                element.attrs.entry(attr.name).or_insert(attr.value);
            }),
            _ => unreachable!("Promise of only passing elements"),
        }
    }

    fn remove_from_parent(&mut self, target: &Self::Handle) {
        self.tree.detach(target).unwrap();
    }

    fn reparent_children(&mut self, node: &Self::Handle, new_parent: &Self::Handle) {
        self.tree.detach(node).unwrap();

        self.tree.make_child(node, new_parent).unwrap();
    }
}
