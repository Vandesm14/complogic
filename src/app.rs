use std::{borrow::Cow, collections::HashMap};

use eframe::egui::{self, Checkbox, TextStyle};
use egui_node_graph::*;

// ========= First, define your user data types =============

/// The NodeData holds a custom data struct inside each node. It's useful to
/// store additional information that doesn't live in parameters. For this
/// example, the node data stores the template (i.e. the "type") of the node.
#[cfg_attr(
  feature = "persistence",
  derive(serde::Serialize, serde::Deserialize)
)]
pub struct NodeData {
  template: NodeTempl,
}

/// `DataType`s are what defines the possible range of connections when
/// attaching two ports together. The graph UI will make sure to not allow
/// attaching incompatible datatypes.
#[derive(PartialEq, Eq)]
#[cfg_attr(
  feature = "persistence",
  derive(serde::Serialize, serde::Deserialize)
)]
pub enum DataType {
  Scalar,
}

/// In the graph, input parameters can optionally have a constant value. This
/// value can be directly edited in a widget inside the node itself.
///
/// There will usually be a correspondence between DataTypes and ValueTypes. But
/// this library makes no attempt to check this consistency. For instance, it is
/// up to the user code in this example to make sure no parameter is created
/// with a DataType of Scalar and a ValueType of Vec2.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(
  feature = "persistence",
  derive(serde::Serialize, serde::Deserialize)
)]
pub enum ValueType {
  Scalar { value: bool },
}

impl Default for ValueType {
  fn default() -> Self {
    // NOTE: This is just a dummy `Default` implementation. The library
    // requires it to circumvent some internal borrow checker issues.
    Self::Scalar { value: false }
  }
}

impl ValueType {
  /// Tries to downcast this value type to a scalar
  pub fn try_to_scalar(self) -> anyhow::Result<bool> {
    let ValueType::Scalar { value } = self;
    Ok(value)
  }
}

/// NodeTemplate is a mechanism to define node templates. It's what the graph
/// will display in the "new node" popup. The user code needs to tell the
/// library how to convert a NodeTemplate into a Node.
#[derive(Clone, Copy)]
#[cfg_attr(
  feature = "persistence",
  derive(serde::Serialize, serde::Deserialize)
)]
pub enum NodeTempl {
  And,
  Immediate,
}

/// The response type is used to encode side-effects produced when drawing a
/// node in the graph. Most side-effects (creating new nodes, deleting existing
/// nodes, handling connections...) are already handled by the library, but this
/// mechanism allows creating additional side effects from user code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MyResponse {
  SetActiveNode(NodeId),
  ClearActiveNode,
}

/// The graph 'global' state. This state struct is passed around to the node and
/// parameter drawing callbacks. The contents of this struct are entirely up to
/// the user. For this example, we use it to keep track of the 'active' node.
#[derive(Default)]
#[cfg_attr(
  feature = "persistence",
  derive(serde::Serialize, serde::Deserialize)
)]
pub struct GraphState {
  pub active_node: Option<NodeId>,
}

// =========== Then, you need to implement some traits ============

// A trait for the data types, to tell the library how to display them
impl DataTypeTrait<GraphState> for DataType {
  fn data_type_color(&self, _user_state: &mut GraphState) -> egui::Color32 {
    match self {
      DataType::Scalar => egui::Color32::from_rgb(38, 109, 211),
    }
  }

  fn name(&self) -> Cow<'_, str> {
    match self {
      DataType::Scalar => Cow::Borrowed("scalar"),
    }
  }
}

// A trait for the node kinds, which tells the library how to build new nodes
// from the templates in the node finder
impl NodeTemplateTrait for NodeTempl {
  type NodeData = NodeData;
  type DataType = DataType;
  type ValueType = ValueType;
  type UserState = GraphState;
  type CategoryType = &'static str;

  fn node_finder_label(
    &self,
    _user_state: &mut Self::UserState,
  ) -> Cow<'_, str> {
    Cow::Borrowed(match self {
      NodeTempl::And => "And Gate",
      NodeTempl::Immediate => "Immediate",
    })
  }

  // this is what allows the library to show collapsible lists in the node finder.
  fn node_finder_categories(
    &self,
    _user_state: &mut Self::UserState,
  ) -> Vec<&'static str> {
    match self {
      NodeTempl::And => vec!["Gate"],
      NodeTempl::Immediate => vec!["Tools"],
    }
  }

  fn node_graph_label(&self, user_state: &mut Self::UserState) -> String {
    // It's okay to delegate this to node_finder_label if you don't want to
    // show different names in the node finder and the node itself.
    self.node_finder_label(user_state).into()
  }

  fn user_data(&self, _user_state: &mut Self::UserState) -> Self::NodeData {
    NodeData { template: *self }
  }

  fn build_node(
    &self,
    graph: &mut Graph<Self::NodeData, Self::DataType, Self::ValueType>,
    _user_state: &mut Self::UserState,
    node_id: NodeId,
  ) {
    // The nodes are created empty by default. This function needs to take
    // care of creating the desired inputs and outputs based on the template

    // We define some closures here to avoid boilerplate. Note that this is
    // entirely optional.
    let input_scalar = |graph: &mut MyGraph, name: &str| {
      graph.add_input_param(
        node_id,
        name.to_string(),
        DataType::Scalar,
        ValueType::Scalar { value: false },
        InputParamKind::ConnectionOrConstant,
        true,
      );
    };

    let output_scalar = |graph: &mut MyGraph, name: &str| {
      graph.add_output_param(node_id, name.to_string(), DataType::Scalar);
    };

    match self {
      NodeTempl::And => {
        // The first input param doesn't use the closure so we can comment
        // it in more detail.
        graph.add_input_param(
          node_id,
          // This is the name of the parameter. Can be later used to
          // retrieve the value. Parameter names should be unique.
          "A".into(),
          // The data type for this input. In this case, a scalar
          DataType::Scalar,
          // The value type for this input. We store zero as default
          ValueType::Scalar { value: false },
          // The input parameter kind. This allows defining whether a
          // parameter accepts input connections and/or an inline
          // widget to set its value.
          InputParamKind::ConnectionOrConstant,
          true,
        );
        input_scalar(graph, "B");
        output_scalar(graph, "out");
      }
      NodeTempl::Immediate => {
        // The first input param doesn't use the closure so we can comment
        // it in more detail.
        graph.add_input_param(
          node_id,
          // This is the name of the parameter. Can be later used to
          // retrieve the value. Parameter names should be unique.
          "A".into(),
          // The data type for this input. In this case, a scalar
          DataType::Scalar,
          // The value type for this input. We store zero as default
          ValueType::Scalar { value: false },
          // The input parameter kind. This allows defining whether a
          // parameter accepts input connections and/or an inline
          // widget to set its value.
          InputParamKind::ConnectionOrConstant,
          true,
        );
        output_scalar(graph, "out");
      }
    }
  }
}

pub struct AllNodeTempls;
impl NodeTemplateIter for AllNodeTempls {
  type Item = NodeTempl;

  fn all_kinds(&self) -> Vec<Self::Item> {
    // This function must return a list of node kinds, which the node finder
    // will use to display it to the user. Crates like strum can reduce the
    // boilerplate in enumerating all variants of an enum.
    vec![NodeTempl::And, NodeTempl::Immediate]
  }
}

impl WidgetValueTrait for ValueType {
  type Response = MyResponse;
  type UserState = GraphState;
  type NodeData = NodeData;
  fn value_widget(
    &mut self,
    param_name: &str,
    _node_id: NodeId,
    ui: &mut egui::Ui,
    _user_state: &mut GraphState,
    _node_data: &NodeData,
  ) -> Vec<MyResponse> {
    // This trait is used to tell the library which UI to display for the
    // inline parameter widgets.
    match self {
      ValueType::Scalar { value } => {
        ui.horizontal(|ui| {
          ui.label(param_name);
          ui.add(Checkbox::new(value, ""));
        });
      }
    }
    // This allows you to return your responses from the inline widgets.
    Vec::new()
  }
}

impl UserResponseTrait for MyResponse {}
impl NodeDataTrait for NodeData {
  type Response = MyResponse;
  type UserState = GraphState;
  type DataType = DataType;
  type ValueType = ValueType;

  // This method will be called when drawing each node. This allows adding
  // extra ui elements inside the nodes. In this case, we create an "active"
  // button which introduces the concept of having an active node in the
  // graph. This is done entirely from user code with no modifications to the
  // node graph library.
  fn bottom_ui(
    &self,
    ui: &mut egui::Ui,
    node_id: NodeId,
    _graph: &Graph<NodeData, DataType, ValueType>,
    user_state: &mut Self::UserState,
  ) -> Vec<NodeResponse<MyResponse, NodeData>>
  where
    MyResponse: UserResponseTrait,
  {
    // This logic is entirely up to the user. In this case, we check if the
    // current node we're drawing is the active one, by comparing against
    // the value stored in the global user state, and draw different button
    // UIs based on that.

    let mut responses = vec![];
    let is_active = user_state
      .active_node
      .map(|id| id == node_id)
      .unwrap_or(false);

    // Pressing the button will emit a custom user response to either set,
    // or clear the active node. These responses do nothing by themselves,
    // the library only makes the responses available to you after the graph
    // has been drawn. See below at the update method for an example.
    if !is_active {
      if ui.button("👁 Set active").clicked() {
        responses.push(NodeResponse::User(MyResponse::SetActiveNode(node_id)));
      }
    } else {
      let button = egui::Button::new(
        egui::RichText::new("👁 Active").color(egui::Color32::BLACK),
      )
      .fill(egui::Color32::GOLD);
      if ui.add(button).clicked() {
        responses.push(NodeResponse::User(MyResponse::ClearActiveNode));
      }
    }

    responses
  }
}

type MyGraph = Graph<NodeData, DataType, ValueType>;
type MyEditorState =
  GraphEditorState<NodeData, DataType, ValueType, NodeTempl, GraphState>;

#[derive(Default)]
pub struct NodeGraphExample {
  // The `GraphEditorState` is the top-level object. You "register" all your
  // custom types by specifying it as its generic parameters.
  state: MyEditorState,

  user_state: GraphState,
}

#[cfg(feature = "persistence")]
const PERSISTENCE_KEY: &str = "egui_node_graph";

#[cfg(feature = "persistence")]
impl NodeGraphExample {
  /// If the persistence feature is enabled, Called once before the first frame.
  /// Load previous app state (if any).
  pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
    let state = cc
      .storage
      .and_then(|storage| eframe::get_value(storage, PERSISTENCE_KEY))
      .unwrap_or_default();
    Self {
      state,
      user_state: GraphState::default(),
    }
  }
}

impl eframe::App for NodeGraphExample {
  #[cfg(feature = "persistence")]
  /// If the persistence function is enabled,
  /// Called by the frame work to save state before shutdown.
  fn save(&mut self, storage: &mut dyn eframe::Storage) {
    eframe::set_value(storage, PERSISTENCE_KEY, &self.state);
  }
  /// Called each time the UI needs repainting, which may be many times per second.
  /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::TopBottomPanel::top("top").show(ctx, |ui| {
      egui::menu::bar(ui, |ui| {
        egui::widgets::global_dark_light_mode_switch(ui);
      });
    });
    let graph_response = egui::CentralPanel::default()
      .show(ctx, |ui| {
        self.state.draw_graph_editor(
          ui,
          AllNodeTempls,
          &mut self.user_state,
          Vec::default(),
        )
      })
      .inner;
    for node_response in graph_response.node_responses {
      // Here, we ignore all other graph events. But you may find
      // some use for them. For example, by playing a sound when a new
      // connection is created
      if let NodeResponse::User(user_event) = node_response {
        match user_event {
          MyResponse::SetActiveNode(node) => {
            self.user_state.active_node = Some(node)
          }
          MyResponse::ClearActiveNode => self.user_state.active_node = None,
        }
      }
    }

    if let Some(node) = self.user_state.active_node {
      if self.state.graph.nodes.contains_key(node) {
        let text =
          match evaluate_node(&self.state.graph, node, &mut HashMap::new()) {
            Ok(value) => format!("The result is: {:?}", value),
            Err(err) => format!("Execution error: {}", err),
          };
        ctx.debug_painter().text(
          egui::pos2(10.0, 35.0),
          egui::Align2::LEFT_TOP,
          text,
          TextStyle::Button.resolve(&ctx.style()),
          egui::Color32::WHITE,
        );
      } else {
        self.user_state.active_node = None;
      }
    }
  }
}

type OutputsCache = HashMap<OutputId, ValueType>;

/// Recursively evaluates all dependencies of this node, then evaluates the node itself.
pub fn evaluate_node(
  graph: &MyGraph,
  node_id: NodeId,
  outputs_cache: &mut OutputsCache,
) -> anyhow::Result<ValueType> {
  // To solve a similar problem as creating node types above, we define an
  // Evaluator as a convenience. It may be overkill for this small example,
  // but something like this makes the code much more readable when the
  // number of nodes starts growing.

  struct Evaluator<'a> {
    graph: &'a MyGraph,
    outputs_cache: &'a mut OutputsCache,
    node_id: NodeId,
  }
  impl<'a> Evaluator<'a> {
    fn new(
      graph: &'a MyGraph,
      outputs_cache: &'a mut OutputsCache,
      node_id: NodeId,
    ) -> Self {
      Self {
        graph,
        outputs_cache,
        node_id,
      }
    }
    fn evaluate_input(&mut self, name: &str) -> anyhow::Result<ValueType> {
      // Calling `evaluate_input` recursively evaluates other nodes in the
      // graph until the input value for a paramater has been computed.
      evaluate_input(self.graph, self.node_id, name, self.outputs_cache)
    }
    fn populate_output(
      &mut self,
      name: &str,
      value: ValueType,
    ) -> anyhow::Result<ValueType> {
      // After computing an output, we don't just return it, but we also
      // populate the outputs cache with it. This ensures the evaluation
      // only ever computes an output once.
      //
      // The return value of the function is the "final" output of the
      // node, the thing we want to get from the evaluation. The example
      // would be slightly more contrived when we had multiple output
      // values, as we would need to choose which of the outputs is the
      // one we want to return. Other outputs could be used as
      // intermediate values.
      //
      // Note that this is just one possible semantic interpretation of
      // the graphs, you can come up with your own evaluation semantics!
      populate_output(self.graph, self.outputs_cache, self.node_id, name, value)
    }
    fn input_scalar(&mut self, name: &str) -> anyhow::Result<bool> {
      self.evaluate_input(name)?.try_to_scalar()
    }
    fn output_scalar(
      &mut self,
      name: &str,
      value: bool,
    ) -> anyhow::Result<ValueType> {
      self.populate_output(name, ValueType::Scalar { value })
    }
  }

  let node = &graph[node_id];
  let mut evaluator = Evaluator::new(graph, outputs_cache, node_id);
  match node.user_data.template {
    NodeTempl::And => {
      let a = evaluator.input_scalar("A")?;
      let b = evaluator.input_scalar("B")?;
      evaluator.output_scalar("out", a && b)
    }
    NodeTempl::Immediate => {
      let a = evaluator.input_scalar("A")?;
      evaluator.output_scalar("out", a)
    }
  }
}

fn populate_output(
  graph: &MyGraph,
  outputs_cache: &mut OutputsCache,
  node_id: NodeId,
  param_name: &str,
  value: ValueType,
) -> anyhow::Result<ValueType> {
  let output_id = graph[node_id].get_output(param_name)?;
  outputs_cache.insert(output_id, value);
  Ok(value)
}

// Evaluates the input value of
fn evaluate_input(
  graph: &MyGraph,
  node_id: NodeId,
  param_name: &str,
  outputs_cache: &mut OutputsCache,
) -> anyhow::Result<ValueType> {
  let input_id = graph[node_id].get_input(param_name)?;

  // The output of another node is connected.
  if let Some(other_output_id) = graph.connection(input_id) {
    // The value was already computed due to the evaluation of some other
    // node. We simply return value from the cache.
    if let Some(other_value) = outputs_cache.get(&other_output_id) {
      Ok(*other_value)
    }
    // This is the first time encountering this node, so we need to
    // recursively evaluate it.
    else {
      // Calling this will populate the cache
      evaluate_node(graph, graph[other_output_id].node, outputs_cache)?;

      // Now that we know the value is cached, return it
      Ok(
        *outputs_cache
          .get(&other_output_id)
          .expect("Cache should be populated"),
      )
    }
  }
  // No existing connection, take the inline value instead.
  else {
    Ok(graph[input_id].value)
  }
}
