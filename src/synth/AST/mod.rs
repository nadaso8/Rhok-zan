use nom;

pub struct SourceText {
    time_units: Option<TimeUnits>,
    descriptions: Vec<Description>,
}

struct TimeUnits {}

enum Description {
    // modules are the only relevant langauge feature for now
    ModuleDeclaration {},
    UDPDeclaration {},
    InterfaceDeclaration {},
    ProgramDeclaration {},
    PackageDeclaration {},
    PackageItem {},
    BindDirective {},
    ConfigDeclaration {},
}
enum ModuleHeader {
    ModuleNoansiHeader {
        attributes: Vec<AttributeInstance>,
        lifetime: Option<Lifetime>,
        identifier: ModuleIdentifier,
        imports: Vec<PackageImport>,
        parameters: Option<ParameterPortList>,
        ports: ListOfPorts,
    },
    ModuleAnsiHeader {
        attributes: Vec<AttributeInstance>,
        lifetime: Option<Lifetime>,
        identifier: ModuleIdentifier,
        imports: Vec<PackageImport>,
        parameters: Option<ParameterPortList>,
        ports: Option<ListOfPortDeclarations>,
    },
}
