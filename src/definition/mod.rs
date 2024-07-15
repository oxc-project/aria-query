use phf::OrderedMap;
#[cfg(test)]
use serde::{ser::SerializeStruct, Serialize};

#[cfg_attr(test, derive(Serialize))]
pub enum ARIAAbstractRole {
    Command,
    Composite,
    Input,
    Landmark,
    Range,
    RoleType,
    Section,
    SectionHead,
    Select,
    Structure,
    Widget,
    Window,
}

#[cfg_attr(test, derive(Serialize))]
pub enum ARIAWidgetRole {
    Button,
    CheckBox,
    GridCell,
    Link,
    MenuItem,
    MenuItemCheckBox,
    MenuItemRadio,
    Option,
    ProgressBar,
    Radio,
    ScrollBar,
    SearchBox,
    Slider,
    SpinButton,
    Switch,
    Tab,
    TabPanel,
    TextBox,
    TreeItem,
}

#[cfg_attr(test, derive(Serialize))]
pub enum ARIACompositeWidgetRole {
    ComboBox,
    Grid,
    ListBox,
    Menu,
    MenuBar,
    RadioGroup,
    TabList,
    Tree,
    TreeGrid,
}

#[cfg_attr(test, derive(Serialize))]
pub enum ARIADocumentStructureRole {
    Application,
    Article,
    BlockQuote,
    Caption,
    Cell,
    ColumnHeader,
    Definition,
    Deletion,
    Directory,
    Document,
    Emphasis,
    Feed,
    Figure,
    Generic,
    Group,
    Heading,
    Img,
    Insertion,
    List,
    ListItem,
    Mark,
    Math,
    Meter,
    None,
    Note,
    Paragraph,
    Presentation,
    Row,
    RowGroup,
    RowHeader,
    Separator,
    Strong,
    Subscript,
    Superscript,
    Table,
    Term,
    Time,
    ToolBar,
    ToolTip,
}

#[cfg_attr(test, derive(Serialize))]
pub enum ARIALandmarkRole {
    Banner,
    Complementary,
    ContentInfo,
    Form,
    Main,
    Navigation,
    Region,
    Search,
}

#[cfg_attr(test, derive(Serialize))]
pub enum ARIALiveRegionRole {
    Alert,
    Log,
    Marquee,
    Status,
    Timer,
}

#[cfg_attr(test, derive(Serialize))]
pub enum ARIAWindowRole {
    AlertDialog,
    Dialog,
}

#[cfg_attr(test, derive(Serialize))]
pub enum ARIAUncategorizedRole {
    Code,
}

// please define enum equal to the above flow type
#[cfg_attr(test, derive(Serialize))]
pub enum ARIADPubRole {
    DocAbstract,
    DocAcknowledgments,
    DocAfterword,
    DocAppendix,
    DocBacklink,
    DocBiblioentry,
    DocBibliography,
    DocBiblioref,
    DocChapter,
    DocColophon,
    DocConclusion,
    DocCover,
    DocCredit,
    DocCredits,
    DocDedication,
    DocEndnote,
    DocEndnotes,
    DocEpigraph,
    DocEpilogue,
    DocErrata,
    DocExample,
    DocFootnote,
    DocForeword,
    DocGlossary,
    DocGlossref,
    DocIndex,
    DocIntroduction,
    DocNoteref,
    DocNotice,
    DocPagebreak,
    DocPagelist,
    DocPart,
    DocPreface,
    DocPrologue,
    DocPullquote,
    DocQna,
    DocSubtitle,
    DocTip,
    DocToc,
}

#[cfg_attr(test, derive(Serialize))]
pub enum ARIAGraphicsRole {
    GraphicsDocument,
    GraphicsObject,
    GraphicsSymbol,
}

#[cfg_attr(test, derive(Serialize))]
pub enum ARIARole {
    ARIAWidgetRole(ARIAWidgetRole),
    ARIACompositeWidgetRole(ARIACompositeWidgetRole),
    ARIADocumentStructureRole(ARIADocumentStructureRole),
    ARIALandmarkRole(ARIALandmarkRole),
    ARIALiveRegionRole(ARIALiveRegionRole),
    ARIAWindowRole(ARIAWindowRole),
    ARIAUncategorizedRole(ARIAUncategorizedRole),
    ARIADPubRole(ARIADPubRole),
    ARIAGraphicsRole(ARIAGraphicsRole),
}

#[cfg_attr(test, derive(Serialize))]
pub enum ARIARoleDefinitionKey {
    ARIAAbstractRole(ARIAAbstractRole),
    ARIARole(ARIARole),
    ARIADPubRole(ARIADPubRole),
    ARIAGraphicsRole(ARIAGraphicsRole),
}

#[cfg_attr(test, derive(Serialize))]
pub enum ARIANameFromSources {
    Author,
    Contents,
    Prohibited,
}

#[cfg_attr(test, derive(Serialize))]
pub enum ARIAState {
    AriaBusy,
    AriaChecked,
    AriaDisabled,
    AriaExpanded,
    AriaGrabbed,
    AriaHidden,
    AriaInvalid,
    AriaPressed,
    AriaSelected,
}

#[cfg_attr(test, derive(Serialize))]
pub enum ARIAProperty {
    AriaActivedescendant,
    AriaAtomic,
    AriaAutocomplete,
    AriaBraillelabel,
    AriaBrailleroledescription,
    AriaColcount,
    AriaColindex,
    AriaColspan,
    AriaControls,
    AriaCurrent,
    AriaDescribedby,
    AriaDescription,
    AriaDetails,
    AriaDropeffect,
    AriaErrormessage,
    AriaFlowto,
    AriaHaspopup,
    AriaKeyshortcuts,
    AriaLabel,
    AriaLabelledby,
    AriaLevel,
    AriaLive,
    AriaModal,
    AriaMultiline,
    AriaMultiselectable,
    AriaOrientation,
    AriaOwns,
    AriaPlaceholder,
    AriaPosinset,
    AriaReadonly,
    AriaRelevant,
    AriaRequired,
    AriaRoledescription,
    AriaRowcount,
    AriaRowindex,
    AriaRowspan,
    AriaSetsize,
    AriaSort,
    AriaValuemax,
    AriaValuemin,
    AriaValuenow,
    AriaValuetext,
    ARIAState(ARIAState),
}

pub enum ARIAPropertyDefinitionType {
    String,
    Id,
    IdList,
    Integer,
    Number,
    Boolean,
    Token,
    TokenList,
    Tristate,
}

pub struct ARIAPropertyDefinition {
    pub key: ARIAProperty,
    pub type_: ARIAPropertyDefinitionType,
    pub values: Option<&'static [&'static str]>,
    pub allow_undefined: Option<bool>,
}

#[cfg(test)]
impl Serialize for ARIAPropertyDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("ARIAProperty", 4)?;
        state.serialize_field(
            "key",
            match self.key {
                ARIAProperty::AriaActivedescendant => "aria-activedescendant",
                ARIAProperty::AriaAtomic => "aria-atomic",
                ARIAProperty::AriaAutocomplete => "aria-autocomplete",
                ARIAProperty::AriaBraillelabel => "aria-braillelabel",
                ARIAProperty::AriaBrailleroledescription => "aria-brailleroledescription",
                ARIAProperty::AriaColcount => "aria-colcount",
                ARIAProperty::AriaColindex => "aria-colindex",
                ARIAProperty::AriaColspan => "aria-colspan",
                ARIAProperty::AriaControls => "aria-controls",
                ARIAProperty::AriaCurrent => "aria-current",
                ARIAProperty::AriaDescribedby => "aria-describedby",
                ARIAProperty::AriaDescription => "aria-description",
                ARIAProperty::AriaDetails => "aria-details",
                ARIAProperty::AriaDropeffect => "aria-dropeffect",
                ARIAProperty::AriaErrormessage => "aria-errormessage",
                ARIAProperty::AriaFlowto => "aria-flowto",
                ARIAProperty::AriaHaspopup => "aria-haspopup",
                ARIAProperty::AriaKeyshortcuts => "aria-keyshortcuts",
                ARIAProperty::AriaLabel => "aria-label",
                ARIAProperty::AriaLabelledby => "aria-labelledby",
                ARIAProperty::AriaLevel => "aria-level",
                ARIAProperty::AriaLive => "aria-live",
                ARIAProperty::AriaModal => "aria-modal",
                ARIAProperty::AriaMultiline => "aria-multiline",
                ARIAProperty::AriaMultiselectable => "aria-multiselectable",
                ARIAProperty::AriaOrientation => "aria-orientation",
                ARIAProperty::AriaOwns => "aria-owns",
                ARIAProperty::AriaPlaceholder => "aria-placeholder",
                ARIAProperty::AriaPosinset => "aria-posinset",
                ARIAProperty::AriaReadonly => "aria-readonly",
                ARIAProperty::AriaRelevant => "aria-relevant",
                ARIAProperty::AriaRequired => "aria-required",
                ARIAProperty::AriaRoledescription => "aria-roledescription",
                ARIAProperty::AriaRowcount => "aria-rowcount",
                ARIAProperty::AriaRowindex => "aria-rowindex",
                ARIAProperty::AriaRowspan => "aria-rowspan",
                ARIAProperty::AriaSetsize => "aria-setsize",
                ARIAProperty::AriaSort => "aria-sort",
                ARIAProperty::AriaValuemax => "aria-valuemax",
                ARIAProperty::AriaValuemin => "aria-valuemin",
                ARIAProperty::AriaValuenow => "aria-valuenow",
                ARIAProperty::AriaValuetext => "aria-valuetext",
                ARIAProperty::ARIAState(ARIAState::AriaBusy) => "aria-busy",
                ARIAProperty::ARIAState(ARIAState::AriaChecked) => "aria-checked",
                ARIAProperty::ARIAState(ARIAState::AriaDisabled) => "aria-disabled",
                ARIAProperty::ARIAState(ARIAState::AriaExpanded) => "aria-expanded",
                ARIAProperty::ARIAState(ARIAState::AriaGrabbed) => "aria-grabbed",
                ARIAProperty::ARIAState(ARIAState::AriaHidden) => "aria-hidden",
                ARIAProperty::ARIAState(ARIAState::AriaInvalid) => "aria-invalid",
                ARIAProperty::ARIAState(ARIAState::AriaPressed) => "aria-pressed",
                ARIAProperty::ARIAState(ARIAState::AriaSelected) => "aria-selected",
            },
        )?;
        state.serialize_field(
            "type",
            match self.type_ {
                ARIAPropertyDefinitionType::String => "string",
                ARIAPropertyDefinitionType::Id => "id",
                ARIAPropertyDefinitionType::IdList => "idlist",
                ARIAPropertyDefinitionType::Integer => "integer",
                ARIAPropertyDefinitionType::Number => "number",
                ARIAPropertyDefinitionType::Boolean => "boolean",
                ARIAPropertyDefinitionType::Token => "token",
                ARIAPropertyDefinitionType::TokenList => "tokenlist",
                ARIAPropertyDefinitionType::Tristate => "tristate",
            },
        )?;
        match &self.values {
            Some(values) => {
                state.serialize_field("values", &values)?;
            }
            None => {
                state.skip_field("values")?;
            }
        };
        match self.allow_undefined {
            Some(allow_undefined) => {
                state.serialize_field("allow_undefined", &allow_undefined)?;
            }
            None => {
                state.skip_field("allow_undefined")?;
            }
        };

        state.end()
    }
}

#[cfg_attr(test, derive(Serialize))]
pub enum ARIAPropertyCurrent {
    Page,
    Step,
    Location,
    Date,
    Time,
    True,
    False,
}

// These constraints are drawn from the mapping between ARIA and HTML:
// https://www.w3.org/TR/html-aria
#[cfg_attr(test, derive(Serialize))]
pub struct ARIARoleRelationConceptAttributeConstraints {
    // The attribute does not exist on the node: <a>
    pub undefined: bool,
    // The attribute has a value: <a b="c">
    pub set: bool,
    pub gt1: bool,
}

#[cfg_attr(test, derive(Serialize))]
pub struct ARIARoleRelationConceptAttribute {
    pub name: String,
    pub value: Option<String>,
    pub constraints: Option<ARIARoleRelationConceptAttributeConstraints>,
}

// These constraints are drawn from the mapping between ARIA and HTML:
// https://www.w3.org/TR/html-aria
#[cfg_attr(test, derive(Serialize))]
pub struct ARIARoleRelationConceptConstraints {
    pub scoped_to_the_body_element: bool,
    pub scoped_to_the_main_element: bool,
    pub scoped_to_a_sectioning_root_element_other_than_body: bool,
    pub scoped_to_a_sectioning_content_element: bool,
    pub direct_descendant_of_document: bool,
    pub direct_descendant_of_ol: bool,
    pub direct_descendant_of_ul: bool,
    pub direct_descendant_of_menu: bool,
    pub direct_descendant_of_details_element_with_the_open_attribute_defined: bool,
    pub ancestor_table_element_has_table_role: bool,
    pub ancestor_table_element_has_grid_role: bool,
    pub ancestor_table_element_has_treegrid_role: bool,
    pub the_size_attribute_value_is_greater_than_1: bool,
    pub the_multiple_attribute_is_not_set_and_the_size_attribute_does_not_have_a_value_greater_than_1:
        bool,
    pub the_list_attribute_is_not_set: bool,
}

/* The concept in a related domain that informs behavior mappings.
** Related domains include: HTML, "Device Independence Delivery Unit", XForms,
** and ARIA to name a few.
**/
pub struct ARIARoleRelationConcept {
    pub name: &'static str,
    pub attributes: Option<Vec<ARIARoleRelationConceptAttribute>>,
    pub constraints: Option<ARIARoleRelationConceptConstraints>,
}

#[cfg(test)]
impl Serialize for ARIARoleRelationConcept {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("ARIARoleRelation", 4)?;

        match &self.attributes {
            Some(values) => {
                state.serialize_field("attributes", &values)?;
            }
            None => {
                state.skip_field("attributes")?;
            }
        };

        match &self.constraints {
            Some(values) => {
                state.serialize_field("constraints", &values)?;
            }
            None => {
                state.skip_field("constraints")?;
            }
        };

        state.end()
    }
}

#[cfg_attr(test, derive(Serialize))]
pub struct ARIARoleRelation {
    pub module: Option<&'static str>,
    pub concept: Option<ARIARoleRelationConcept>,
}

pub type ARIAProps = OrderedMap<&'static str, &'static Option<ARIAPropertyCurrent>>;

pub struct ARIARoleDefinition {
    /* Abstract roles may not be used in HTML. */
    pub is_abstract: bool,
    pub accessible_name_required: bool,
    /* The concepts in related domains that inform behavior mappings. */
    pub base_concepts: Option<&'static [&'static ARIARoleRelation]>,
    /* Child presentational roles strip child nodes of roles and flatten the content to text. */
    pub children_presentational: bool,
    pub name_from: Option<&'static [&'static ARIANameFromSources]>,
    /* aria-* properties and states disallowed on this role. */
    pub prohibited_props: Option<&'static [&'static ARIAProperty]>,
    /* aria-* properties and states allowed on this role. */
    pub props: Option<&'static ARIAProps>,
    /* The concepts in related domains that inform behavior mappings. */
    pub related_concepts: Option<&'static [ARIARoleRelation]>,
    pub require_context_role: Option<&'static [&'static ARIARole]>,
    pub required_context_role: Option<&'static [&'static ARIARole]>,
    pub required_owned_elements: Option<&'static [&'static [&'static ARIARole]]>,
    /* aria-* properties and states required on this role. */
    pub required_props: Option<&'static ARIAProps>,
    /* An array or super class "stacks." Each stack contains a LIFO list of
     ** strings correspond to a super class in the inheritance chain of this
     ** role. Roles may have more than one inheritance chain, which is why
     ** this property is an array of arrays and not a single array. */
    pub super_class: Option<&'static [&'static [&'static ARIARoleDefinitionKey]]>,
}

#[cfg(test)]
impl Serialize for ARIARoleDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("ARIARoleDefinition", 4)?;

        match &self.base_concepts {
            Some(values) => {
                state.serialize_field("base_concepts", &values)?;
            }
            None => {
                state.skip_field("base_concepts")?;
            }
        };

        match &self.name_from {
            Some(values) => {
                state.serialize_field("name_from", &values)?;
            }
            None => {
                state.skip_field("name_from")?;
            }
        };

        match &self.prohibited_props {
            Some(values) => {
                state.serialize_field("prohibited_props", &values)?;
            }
            None => {
                state.skip_field("prohibited_props")?;
            }
        };

        match &self.related_concepts {
            Some(values) => {
                state.serialize_field("related_concepts", &values)?;
            }
            None => {
                state.skip_field("related_concepts")?;
            }
        };

        state.end()
    }
}

pub type RoleDefinitions = OrderedMap<&'static str, &'static ARIARoleDefinition>;
