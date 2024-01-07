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
    pub type_: ARIAPropertyDefinitionType,
    pub values: Option<Vec<String>>,
    pub allow_undefined: Option<bool>,
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
    pub the_multiple_attribute_is_not_set_and_the_size_attribute_does_not_have_a_value_greater_than_1: bool,
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
    pub prohibited_props: Option<Vec<ARIAProperty>>,
    /* aria-* properties and states allowed on this role. */
    pub props: Option<Vec<ARIAProperty>>,
    /* The concepts in related domains that inform behavior mappings. */
    pub related_concepts: Option<Vec<ARIARoleRelation>>,
    pub require_context_role: Option<Vec<ARIARole>>,
    pub required_context_role: Option<Vec<ARIARole>>,
    pub required_owned_elements: Option<Vec<Vec<String>>>,
    /* aria-* properties and states required on this role. */
    pub required_props: Option<Vec<ARIAProperty>>,
    /* An array or super class "stacks." Each stack contains a LIFO list of
    ** strings correspond to a super class in the inheritance chain of this
    ** role. Roles may have more than one inheritance chain, which is why
    ** this property is an array of arrays and not a single array. */
    pub super_class: Option<Vec<Vec<ARIARoleDefinitionKey>>>,
}

pub type RoleDefinitionTuple = (ARIARoleDefinitionKey, ARIARoleDefinition);
