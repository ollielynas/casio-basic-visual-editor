use egui::{Color32, Ui};
use macroquad::{prelude::*, ui};

use std::{collections::HashMap, string};


#[derive(Debug, Eq, PartialEq, Clone)]
enum Action {
    Add, 
    Remove,
    Edit,
    Move,
}

#[derive(Clone)]
enum Code {
    Main {
        body: Vec<Code>,
    },
    Function {
        name: String,
        body: Vec<Code>,
    },
    CallFunction {
        name: String,
    },
    AssignFloat {
        variable: FloatVariable,
        value: FloatOption,
    },
    AssignString {
        variable: StringVariable,
        value: StringOption,
    },
    If {
        condition: BoolOption,
        body: Vec<Code>,
    },
    While {
        condition: BoolOption,
        body: Vec<Code>,
    },
    Print {
        value: StringOption,
    },
    PrintFloat {
        value: FloatOption,
    },

    End,

}

fn add_code(ui: &mut Ui, mut drag_code: &mut Option<Code>) {
    egui::Frame::group(&egui::Style::default()).fill(Color32::from_rgb(210, 210, 210)).rounding(10.0).show(ui, |ui|  {
        match drag_code {
            Some(c) => {
                ui.horizontal(|ui| {
                    ui.label("Add: ");
                c.render(ui, &mut None, &Action::Edit, &mut HashMap::new(), &mut HashMap::new(), &mut HashMap::new());
                });
            }
            _ => {}
        }
    });
}

impl Code {
    fn render(&mut self, ui: &mut Ui, mut drag_code: &mut Option<Code>, action: &Action, string_variables: &mut HashMap<String, StringVariable>, float_variables: &mut HashMap<String, FloatVariable>, bool_variables: &mut HashMap<String, BoolVariable>) {
        match self {
            Code::Main { body } => {
                let mut insert:Option<usize> = None;
                let mut remove:Option<usize> = None;
                let mut move_:Option<usize> = None;
                let mut move_value: Option<Code> = None;
                egui::Frame::group(&egui::Style::default()).fill(Color32::from_rgb(230, 210, 210)).rounding(10.0).show(ui, |ui|  {
                    ui.vertical(|ui| {
                        if ui.label("Main").hovered() && !drag_code.is_none() {
                            if action == &Action::Add {
                                add_code(ui, drag_code);
                            }
                            ui.ctx().input(|i| {
                                if i.pointer.any_click() {
                                    match action {
                                                Action::Add => {insert = Some(0)},
                                                Action::Remove => {remove = Some(0)},
                                                Action::Move => {
                                                    move_ = Some(0);
                                                },
                                                Action::Edit => {},
                                            }
                                }
                            });
                        }
                        ui.vertical(|ui| {
                        
                            for (l,code) in body.iter_mut().enumerate() {
                                if ui.horizontal(|ui| {
                                    code.render(ui, drag_code, action, string_variables, float_variables, bool_variables);
                                }).response.hovered() && !drag_code.is_none() && match code {
                                    Code::Function { name: _, body: _ } => false,
                                    Code::Main { body: _ } => false,
                                    Code::If { condition: _, body: _ } => false,
                                    Code::While { condition: _, body: _ } => false,

                                    _ => true
                                } {
                                    if action == &Action::Add {
                                add_code(ui, drag_code);
                            }
                                    ui.ctx().input(|i| {
                                        if i.pointer.any_click() {
                                            match action {
                                                Action::Add => {insert = Some(l)},
                                                Action::Remove => {remove = Some(l)},
                                                Action::Move => {
                                                    move_ = Some(l);
                                                },
                                                Action::Edit => {},
                                            }
                                        }
                                    });
                                }
                        
                            }
                        });
                    });
                });
                if let Some(l) = insert {
                    if body.len() == 0 {
                        body.push(drag_code.take().unwrap());
                }else {
                    body.insert(l+1, drag_code.take().unwrap());
                }
                }else if let Some(l) = remove {
                    body.remove(l);
                }else if let Some(c) = move_value {
                drag_code.replace(c);
            }
            }
            Code::Print { value } => {
                egui::Frame::group(&egui::Style::default()).fill(Color32::from_rgb(210, 210, 230)).rounding(10.0).show(ui, |ui|  {
                    ui.horizontal(|ui| {
                        ui.label("Print: ");
                        value.render(ui, string_variables);
                    });
                });
            }
            Code::If { condition, body } => {
                let mut insert:Option<usize> = None;
                let mut remove:Option<usize> = None;
                let mut move_:Option<usize> = None;
                let mut move_value: Option<Code> = None;
                egui::Frame::group(&egui::Style::default()).fill(Color32::from_rgb(230, 230, 210)).rounding(10.0).show(ui, |ui|  {
                    ui.vertical(|ui| {
                    if ui.horizontal(|ui| {
                        ui.label("If: ");
                        condition.render(ui, string_variables, float_variables, bool_variables);
                    }).response.hovered() && !drag_code.is_none()   {
                            if action == &Action::Add {
                                add_code(ui, drag_code);
                            }
                            ui.ctx().input(|i| {
                                if i.pointer.any_click() {
                                    match action {
                                                Action::Add => {insert = Some(0)},
                                                Action::Remove => {remove = Some(0)},
                                                Action::Move => {
                                                    move_ = Some(0);
                                                },
                                                Action::Edit => {},
                                            }
                                }
                            });
                        };
                    let mut insert:Option<usize> = None;
                        
                            for (l,code) in body.iter_mut().enumerate() {
                                if ui.horizontal(|ui| {
                                    code.render(ui, drag_code, action, string_variables, float_variables, bool_variables);
                                }).response.hovered() && !drag_code.is_none() & match code {
                                    Code::Function { name: _, body: _ } => false,
                                    Code::Main { body: _ } => false,
                                    Code::If { condition: _, body: _ } => false,
                                    Code::While { condition: _, body: _ } => false,

                                    _ => true
                                }  {
                                    if action == &Action::Add {
                                add_code(ui, drag_code);
                            }
                                    ui.ctx().input(|i| {
                                        if i.pointer.any_click() {
                                            match action {
                                                Action::Add => {insert = Some(l)},
                                                Action::Remove => {remove = Some(l)},
                                                Action::Move => {
                                                    move_ = Some(l);
                                                },
                                                Action::Edit => {},
                                            }
                                        }
                                    });
                                }
                        
                            }
                });
                if let Some(l) = insert {
                    if body.len() == 0 {
                        body.push(drag_code.take().unwrap());
                }else {
                    body.insert(l+1, drag_code.take().unwrap());
                }
                }else if let Some(l) = remove {
                    body.remove(l);
                }else if let Some(l) = move_ {
                    
                    move_value = Some(body.remove(l));
                        }
                });
                if let Some(c) = move_value {
            drag_code.replace(c);
        }
            }
            Code::PrintFloat { value } => {
                egui::Frame::group(&egui::Style::default()).fill(Color32::from_rgb(210, 210, 230)).rounding(10.0).show(ui, |ui|  {
                    ui.horizontal(|ui| {
                        ui.label("Print Float: ");
                        value.render(ui, float_variables);
                    });
                });
            }

            Code::AssignFloat { variable, value } => {
                egui::Frame::group(&egui::Style::default()).fill(Color32::from_rgb(210, 210, 230)).rounding(10.0).show(ui, |ui|  {
                    ui.horizontal(|ui| {
                        ui.label("Assign Float: ");
                        ui.label(" = ");
                        value.render(ui, float_variables);
                    });
                });
            }

            _ => {
                egui::Frame::group(&egui::Style::default()).fill(Color32::from_rgb(210, 210, 210)).rounding(10.0).show(ui, |ui|  {
                    ui.label("not yet added");
                });
            }
        }

        

    }


    
}

#[derive(Clone)]
struct MathString {
    value: String,
}

#[derive(Clone)]
enum FloatOption {
    MathString(MathString),
    Variable(FloatVariable),
    Float(f32),
}

impl FloatOption {
    fn render(&mut self, ui: &mut Ui, float_variables: &mut HashMap<String, FloatVariable>) {
        ui.horizontal(|ui| {
        ui.menu_button(match self {
            FloatOption::MathString(_math_string) => "Math String",
            FloatOption::Variable(_float_variable) => "Variable",
            FloatOption::Float(_float) => "Float",
        }, |ui| {
            if ui.small_button("Math String").clicked() {
                *self = FloatOption::MathString(MathString {
                    value: "".to_owned(),
                });
            }
            if ui.small_button("Variable").clicked() {
                *self = FloatOption::Variable(FloatVariable {
                    name: "Default".to_owned(),
                    value: 0.0,
                });
            }
            if ui.small_button("Float").clicked() {
                *self = FloatOption::Float(0.0);
            }
    });
        match self {
            FloatOption::MathString(math_string) => {
                ui.text_edit_singleline(&mut math_string.value);
            }
            FloatOption::Variable(float_variable) => {
                let mut new_value:Option<FloatVariable> = None;
                ui.menu_button(&float_variable.name, |ui| {
                    for (name, variable) in float_variables.iter_mut() {
                        if ui.small_button(name).clicked() {
                            new_value = Some(variable.clone());
                        }
                    }
                });
                if let Some(variable) = new_value {
                    *float_variable = variable;
                }
            }
            FloatOption::Float(float) => {
                ui.add(egui::DragValue::new(float));
            }
        }
        });

    }
    
    fn string(&self) -> String {
        return match self {
            FloatOption::MathString(math_string) => math_string.value.clone(),
            FloatOption::Variable(float_variable) => float_variable.name.clone(),
            FloatOption::Float(float) => float.to_string(),
    }
    
    }
    fn float(&self) -> f32 {
        return match self {
            FloatOption::MathString(math_string) => math_string.value.parse::<f32>().unwrap_or(0.0),
            FloatOption::Variable(float_variable) => float_variable.value,
            FloatOption::Float(float) => *float,
        }
    }
}

#[derive(Clone)]
struct BoolVariable {
    name: String,
    value: bool,
}


#[derive(Clone)]
enum BoolOption {
    Variable(BoolVariable),
    StringEqual(StringOption, StringOption),
    Equal(FloatOption, FloatOption),
    NotEqual(FloatOption, FloatOption),
    Less(FloatOption, FloatOption),
    LessEqual(FloatOption, FloatOption),
    Greater(FloatOption, FloatOption),
    GreaterEqual(FloatOption, FloatOption),
    True,
    False,
}



impl BoolOption {
    fn render(&mut self, ui: &mut Ui, string_variables: &mut HashMap<String, StringVariable>, float_variables: &mut HashMap<String, FloatVariable>, bool_variables: &mut HashMap<String, BoolVariable>) {
        ui.horizontal(|ui| {
        match self {
            BoolOption::StringEqual(a, _) => a.render(ui, string_variables),
            BoolOption::Equal(a, _) => a.render(ui, float_variables),
            BoolOption::NotEqual(a, _) => a.render(ui, float_variables),
            BoolOption::Less(a, _) => a.render(ui, float_variables),
            BoolOption::LessEqual(a, _) => a.render(ui, float_variables),
            BoolOption::Greater(a, _) => a.render(ui, float_variables),
            BoolOption::GreaterEqual(a, _) => a.render(ui, float_variables),
            _ => {}
        }
        ui.menu_button(match self {
            BoolOption::Variable(_bool_variable) => "Variable",
            BoolOption::StringEqual(_string1, _string2) => "=",
            BoolOption::Equal(_float1, _float2) => "Equal",
            BoolOption::NotEqual(_float1, _float2) => "Not Equal",
            BoolOption::Less(_float1, _float2) => "Less than",
            BoolOption::LessEqual(_float1, _float2) => "Less or Equal",
            BoolOption::Greater(_float1, _float2) => "Greater than",
            BoolOption::GreaterEqual(_float1, _float2) => "Greater or Equal",
            BoolOption::True => "True",
            BoolOption::False => "False",
        }, |ui| {
            if ui.small_button("Variable").clicked() {
                *self = BoolOption::Variable(BoolVariable {
                    name: "Default".to_owned(),
                    value: false,
                });
            }
            if ui.small_button("=").clicked() {
                *self = BoolOption::StringEqual(StringOption::StringConstant("".to_owned()), StringOption::StringConstant("".to_owned()));
            }
            if ui.small_button("Equal").clicked() {
                *self = BoolOption::Equal(FloatOption::Float(0.0), FloatOption::Float(0.0));
            }
            if ui.small_button("Not Equal").clicked() {
                *self = BoolOption::NotEqual(FloatOption::Float(0.0), FloatOption::Float(0.0));
            }
            if ui.small_button("Less than").clicked() {
                *self = BoolOption::Less(FloatOption::Float(0.0), FloatOption::Float(0.0));
            }
            if ui.small_button("Less or Equal").clicked() {
                *self = BoolOption::LessEqual(FloatOption::Float(0.0), FloatOption::Float(0.0));
            }
            if ui.small_button("Greater than").clicked() {
                *self = BoolOption::Greater(FloatOption::Float(0.0), FloatOption::Float(0.0));
            }
            if ui.small_button("Greater or Equal").clicked() {
                *self = BoolOption::GreaterEqual(FloatOption::Float(0.0), FloatOption::Float(0.0));
            }
            if ui.small_button("True").clicked() {
                *self = BoolOption::True;
            }
            if ui.small_button("False").clicked() {
                *self = BoolOption::False;
            }
    });
    match self {
            BoolOption::Variable(bool_variable) => {
                let mut new_value:Option<BoolVariable> = None;
                ui.menu_button(&bool_variable.name, |ui| {
                    for (name, bool_variable) in bool_variables.iter_mut() {
                        if ui.small_button(name).clicked() {
                            new_value = Some(bool_variable.clone());
                        }
                    }
                });
                if let Some(new_value) = new_value {
                    *bool_variable = new_value;
                }
            }
            BoolOption::StringEqual(_, b) => b.render(ui, string_variables),
            BoolOption::Equal(_, b) => b.render(ui, float_variables),
            BoolOption::NotEqual(_, b) => b.render(ui, float_variables),
            BoolOption::Less(_, b) => b.render(ui, float_variables),
            BoolOption::LessEqual(_, b) => b.render(ui, float_variables),
            BoolOption::Greater(_, b) => b.render(ui, float_variables),
            BoolOption::GreaterEqual(_, b) => b.render(ui, float_variables),
            _ => {},
        }
        });
    }
}
#[derive(Clone)]
struct FloatVariable {
    name: String,
    value: f32,
}

#[derive(Clone)]
struct StringVariable {
    name: String,
    value: String,
}

#[derive(Clone)]
enum StringOption {
    StringConstant(String),
    StringVariable(StringVariable),
}

impl StringOption {
    fn render(&mut self, ui: &mut Ui, string_variables: &mut HashMap<String, StringVariable>) {
        ui.horizontal(|ui| {
        ui.menu_button(match self {
            StringOption::StringConstant(_string) => "Constant",
            StringOption::StringVariable(_string_variable) => "Variable",
        }, |ui| {
            if ui.small_button("Constant").clicked() {
                *self = StringOption::StringConstant("".to_owned());
            }
            if ui.small_button("Variable").clicked() {
                *self = StringOption::StringVariable(StringVariable {
                    name: "Default".to_owned(),
                    value: "".to_owned(),
                });
            }
    });
        match self {
            StringOption::StringConstant(string) => {
                ui.text_edit_singleline(string);
            }
            StringOption::StringVariable(string_variable) => {
                let mut new_value:Option<StringVariable> = None;
                ui.menu_button(&string_variable.name, |ui| {
                    for (name, string_variable) in string_variables.iter_mut() {
                        if ui.small_button(name).clicked() {
                            new_value = Some(string_variable.clone());
                        }
                    }
                });
                if let Some(new_value) = new_value {
                    *string_variable = new_value;
                }
            }
        }
        });
    }

    fn string(&self) -> String {
        return match self {
            StringOption::StringConstant(string) => string.clone(),
            StringOption::StringVariable(string_variable) => string_variable.value.clone(),
        }
    }
}
#[derive(Clone)]
struct Program {
    Name: String,
    Code: Vec<Code>,
}

impl Default for Program {
    fn default() -> Self {
        Program {
            Name: "Unnamed Program".to_string(),
            Code: vec![Code::Main { body: vec![
                Code::Print { value: StringOption::StringConstant("Hello World".to_owned()) }
            ] }],
        }
    }

    
}



impl Program {
    fn from_string(string: String) -> Self {
        let mut p =Program::default();
        p.Name = string;
        return p;
    }

    fn output(&self) -> String {
        let mut out = "".to_owned();

        out.push_str("TODO");

        return out;
    }

    fn render(&mut self, egui_ctx: &egui::Context, drag_code: &mut Option<Code>, action: &Action, string_variables: &mut HashMap<String, StringVariable>, float_variables: &mut HashMap<String, FloatVariable>, bool_variables: &mut HashMap<String, BoolVariable>) {
        egui::CentralPanel::default().show(egui_ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
            for function in self.Code.iter_mut() {
                function.render(ui, drag_code, action, string_variables, float_variables, bool_variables);
            }
            });
        });

    }
}

#[macroquad::main("egui with macroquad")]
async fn main() {


    
    egui_macroquad::ui(|egui_ctx| {
        egui_ctx.set_visuals(egui::Visuals::light());
    });

    let mut main_menu = true;
    let mut new_program_name = "Unnamed Program".to_owned();

    let mut program = Program::default();

    let mut action = Action::Edit;

    let mut string_variables: HashMap<String, StringVariable> = HashMap::new();
    string_variables.insert("Default".to_owned(), StringVariable {
        name: "Default".to_owned(),
        value: "Default String".to_owned(),
    });
    let mut new_string_variable_name = "Unnamed Variable".to_owned();

    let mut float_variables: HashMap<String, FloatVariable> = HashMap::new();
    let mut new_float_variable_name = "Unnamed Variable".to_owned();
    float_variables.insert("Default".to_owned(), FloatVariable {
        name: "Default".to_owned(),
        value: 0.0,
    });

    let mut bool_variables: HashMap<String, BoolVariable> = HashMap::new();
    let mut new_bool_variable_name = "Unnamed Variable".to_owned();
    bool_variables.insert("Default".to_owned(), BoolVariable {
        name: "Default".to_owned(),
        value: true,
    });

    let mut compiled_text = String::new();

    let mut drag_code: Option<Code> = None;

    loop {
        clear_background(WHITE);

        egui_macroquad::ui(|egui_ctx| {

            if main_menu {

                egui::CentralPanel::default().show(egui_ctx, |ui| {
                    
                    ui.heading("Main Menu");
                    egui::ScrollArea::vertical()
                    
                    .show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.horizontal(|ui| {
                            
                            ui.text_edit_singleline(&mut new_program_name);
                            if ui.button(" Add +").clicked() {
                                main_menu = false;
                                program = Program::from_string(new_program_name.clone());
                            }
                            });
                        });
                    });
                });


            }else {

            egui::TopBottomPanel::top("top")
            .resizable(false).show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(&program.Name);
                    if ui.button("Main Menu").clicked() {
                        main_menu = true;
                    }
                });
            
            });

            egui::SidePanel::left("my_side_panel").show(egui_ctx, |ui| {
                ui.radio_value(&mut action, Action::Edit, "Edit");
                ui.radio_value(&mut action, Action::Add, "Add");
                ui.radio_value(&mut action, Action::Remove, "Remove");
                ui.radio_value(&mut action, Action::Move, "Move");

                ui.separator();
                let mut remove:Option<String> = None;

                ui.label("String Variables:");
                for (name, string_variable) in string_variables.iter_mut() {
                    ui.horizontal(|ui| {
                        ui.label(name);
                        ui.text_edit_singleline(&mut string_variable.value);
                        if ui.button("X").on_hover_text("Remove Variable").clicked() {
                            remove = Some(name.clone());
                        }
                    });
                }
                if let Some(name) = remove {
                    string_variables.remove(&name);
                }
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut new_string_variable_name);
                    if ui.button("Add String Variable").clicked() {
                        string_variables.insert(new_string_variable_name.clone(), StringVariable {
                            name: new_string_variable_name.clone(),
                            value: "".to_owned(),
                        });
                    }
                });
                let mut remove:Option<String> = None;
                ui.separator();
                ui.label("Float Variables:");
                for (name, float_variable) in float_variables.iter_mut() {
                    ui.horizontal(|ui| {
                        ui.label(name);
                        ui.add(egui::DragValue::new(&mut float_variable.value).max_decimals_opt(Some(7)));
                        if ui.button("X").on_hover_text("Remove Variable").clicked() {
                            remove = Some(name.clone());
                        }
                    });
                }
                if let Some(name) = remove {
                    float_variables.remove(&name);
                }
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut new_float_variable_name);
                    if ui.button("Add Float Variable").clicked() {
                        float_variables.insert(new_float_variable_name.clone(), FloatVariable {
                            name: new_float_variable_name.clone(),
                            value: 0.0,
                        });
                    }
                });

                ui.separator();
                if action == Action::Add {
                    ui.label("Selected Code Block:");
                    add_code(ui,&mut drag_code);
                    ui.separator();
                }
                for c in [
                    Code::Print { value: StringOption::StringConstant("Hello World".to_owned()) },
                    Code::If { condition: BoolOption::True, body: vec![] },
                    // Code::AssignFloat { variable: , value: () }

                ].iter_mut() {
                    let rect = ui.horizontal(|ui| {
                        c.render(ui, &mut drag_code, &Action::Edit, &mut HashMap::new(), &mut HashMap::new(), &mut HashMap::new());
                    }).response.rect;
                    if ui.put(rect, egui::Button::new("").fill(egui::Color32::TRANSPARENT).frame(false)).clicked() {
                        drag_code = Some(c.clone());
                    };
                }
            });

            egui::SidePanel::right("output").show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Output");
                    if ui.button("Recompile").clicked() {
                        compiled_text = program.output();
                    }
                    if ui.button("Copy To Clipboard").clicked() {
                        egui_ctx.output_mut(|o| {
                            o.copied_text = compiled_text.clone();
                        }
                        );
                    }
                });
                ui.separator();
                ui.label(&compiled_text);

            });

            egui::Area::new("Drag").order(egui::Order::Tooltip).interactable(false)
            .fixed_pos(match egui_ctx.pointer_interact_pos() {
                Some(pos) => pos,
                None => egui::Pos2::new(0.0, 0.0),
            })
            .show(egui_ctx, |ui| {
                match &mut drag_code {
                    Some(code) => {
                        // code.render(ui, &mut None);
                    }
                    None => {
                    }
                }
            });

            program.render(egui_ctx, &mut drag_code, &action, &mut string_variables, &mut float_variables, &mut bool_variables);
        }
        });

        // Draw things before egui

        

        egui_macroquad::draw();
        
        // Draw things after egui

        next_frame().await;
    }
}