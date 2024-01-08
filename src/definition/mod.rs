#[cfg(test)]
use serde::{ser::SerializeStruct, Serialize};

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

pub enum ARIALiveRegionRole {
    Alert,
    Log,
    Marquee,
    Status,
    Timer,
}

pub enum ARIAWindowRole {
    AlertDialog,
    Dialog,
}

pub enum ARIAUncategorizedRole {
    Code,
}

// please define enum equal to the above flow type
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

pub enum ARIAGraphicsRole {
    GraphicsDocument,
    GraphicsObject,
    GraphicsSymbol,
}

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

pub enum ARIARoleDefinitionKey {
    ARIAAbstractRole(ARIAAbstractRole),
    ARIARole(ARIARole),
    ARIADPubRole(ARIADPubRole),
    ARIAGraphicsRole(ARIAGraphicsRole),
}

pub enum ARIANameFromSources {
    Author,
    Contents,
    Prohibited,
}

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
pub struct ARIARoleRelationConceptAttributeConstraints {
    // The attribute does not exist on the node: <a>
    pub undefined: bool,
    // The attribute has a value: <a b="c">
    pub set: bool,
    pub gt1: bool,
}

pub struct ARIARoleRelationConceptAttribute {
    pub name: String,
    pub value: Option<String>,
    pub constraints: Option<ARIARoleRelationConceptAttributeConstraints>,
}

// These constraints are drawn from the mapping between ARIA and HTML:
// https://www.w3.org/TR/html-aria
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
    pub name: String,
    pub attributes: Option<Vec<ARIARoleRelationConceptAttribute>>,
    pub constraints: Option<ARIARoleRelationConceptConstraints>,
}

pub struct ARIARoleRelation {
    pub module: Option<String>,
    pub concept: Option<ARIARoleRelationConcept>,
}

pub struct ARIARoleDefinition {
    /* Abstract roles may not be used in HTML. */
    pub is_abstract: bool,
    pub accessible_name_required: bool,
    /* The concepts in related domains that inform behavior mappings. */
    pub base_concepts: Option<Vec<ARIARoleRelation>>,
    /* Child presentational roles strip child nodes of roles and flatten the content to text. */
    pub children_presentational: bool,
    pub name_from: Option<Vec<ARIANameFromSources>>,
    /* aria-* properties and states disallowed on this role. */
    pub prohibited_props: Option<Vec<ARIAPropertyMap>>,
    /* aria-* properties and states allowed on this role. */
    pub props: Option<Vec<ARIAPropertyMap>>,
    /* The concepts in related domains that inform behavior mappings. */
    pub related_concepts: Option<Vec<ARIARoleRelation>>,
    pub require_context_role: Option<Vec<ARIARole>>,
    pub required_context_role: Option<Vec<ARIARole>>,
    pub required_owned_elements: Option<Vec<Vec<String>>>,
    /* aria-* properties and states required on this role. */
    pub required_props: Option<Vec<ARIAPropertyMap>>,
    /* An array or super class "stacks." Each stack contains a LIFO list of
     ** strings correspond to a super class in the inheritance chain of this
     ** role. Roles may have more than one inheritance chain, which is why
     ** this property is an array of arrays and not a single array. */
    pub super_class: Option<Vec<Vec<ARIARoleDefinitionKey>>>,
}

pub type RoleDefinitionTuple = (ARIARoleDefinitionKey, ARIARoleDefinition);
pub type RoleDefinitions = Vec<RoleDefinitionTuple>;

pub struct ARIAPropertyMap {
    pub aria_activedescendant: Option<String>,
    pub aria_atomic: Option<String>,
    pub aria_autocomplete: Option<String>,
    pub aria_braillelabel: Option<String>,
    pub aria_brailleroledescription: Option<String>,
    pub aria_busy: Option<String>,
    pub aria_checked: Option<String>,
    pub aria_colcount: Option<String>,
    pub aria_colindex: Option<String>,
    pub aria_colspan: Option<String>,
    pub aria_controls: Option<String>,
    pub aria_current: Option<ARIAPropertyCurrent>,
    pub aria_describedby: Option<String>,
    pub aria_description: Option<String>,
    pub aria_details: Option<String>,
    pub aria_disabled: Option<String>,
    pub aria_dropeffect: Option<String>,
    pub aria_errormessage: Option<String>,
    pub aria_expanded: Option<String>,
    pub aria_flowto: Option<String>,
    pub aria_grabbed: Option<String>,
    pub aria_haspopup: Option<String>,
    pub aria_hidden: Option<String>,
    pub aria_invalid: Option<String>,
    pub aria_keyshortcuts: Option<String>,
    pub aria_label: Option<String>,
    pub aria_labelledby: Option<String>,
    pub aria_level: Option<String>,
    pub aria_live: Option<String>,
    pub aria_modal: Option<String>,
    pub aria_multiline: Option<String>,
    pub aria_multiselectable: Option<String>,
    pub aria_orientation: Option<String>,
    pub aria_owns: Option<String>,
    pub aria_placeholder: Option<String>,
    pub aria_posinset: Option<String>,
    pub aria_pressed: Option<String>,
    pub aria_readonly: Option<String>,
    pub aria_relevant: Option<String>,
    pub aria_required: Option<String>,
    pub aria_roledescription: Option<String>,
    pub aria_rowcount: Option<String>,
    pub aria_rowindex: Option<String>,
    pub aria_rowspan: Option<String>,
    pub aria_selected: Option<String>,
    pub aria_setsize: Option<String>,
    pub aria_sort: Option<String>,
    pub aria_valuemax: Option<String>,
    pub aria_valuemin: Option<String>,
    pub aria_valuenow: Option<String>,
    pub aria_valuetext: Option<String>,
}
