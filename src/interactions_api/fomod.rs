
pub mod types {
    use std::path::PathBuf;


    struct FlagDependency {
        flag: String,
        value: String
    }

    enum Operand {
        And(Vec<FlagDependency>,Box<Vec<Operand>>)
    }

    struct DependencyOperand {
        dep_type: Operand,
    }

    enum Type {
        Optional
    }

    struct TypeDescriptor {
        desc_type: Type
    }

    struct Flag {
        name: String,
        body: String
    }

    struct Folder {
        source: PathBuf,
        destination: String,
        priority: i16
    }

    struct Plugin {
        name: String,
        description: String,
        image: Image,
        conditon_flags: Option<Vec<Flag>>,
        type_descriptor: TypeDescriptor,
        files: Option<Vec<Folder>>
    }

    struct Plugins {
        order: Order,
        plugins: Vec<Plugin>
    }

    enum SelectionType {
        SelectExactlyOne
    }
    struct Group {
        name: String,
        selection_type: SelectionType,
        plugins: Plugins
    }

    struct OptionalFileGroups {
        order: Order,
        group: Group
    }

    enum Order {
        Explicit
    }
    
    struct InstallStep {
        name: String,
        visible: DependencyOperand,
        optional_file_groups: OptionalFileGroups
    }
    
    struct InstallSteps {
        order: Order,
        steps: Vec<InstallStep>
    }
    
    struct Image {
        image_path: String
    }
    
    pub struct FomodData {
        module_name: String,
        module_image: Image
    }
}