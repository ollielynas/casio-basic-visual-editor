use egui::{Color32, Ui, Response, Pos2, util::hash, Label};
use macroquad::{prelude::*, ui};

use serde::{Serialize, Deserialize};


use std::{collections::HashMap, fs::{self, File}, time::Instant, io::Write};



#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
enum Action {
    Add, 
    Remove,
    Edit,
    Move,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    AssignBool {
        variable: BoolVariable,
        value: BoolOption,
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


    Break,

}

fn add_code(ui: &mut Ui, mut drag_code: &mut Option<Code>) {
    egui::Frame::group(&egui::Style::default()).fill(Color32::from_rgb(210, 210, 210)).rounding(10.0).show(ui, |ui|  {
        match drag_code {
            Some(c) => {
                ui.horizontal(|ui| {
                    ui.label("Add: ");
                c.render(ui, &mut Data::default());
                });
            }
            _ => {}
        }
    });
}

impl Code {

    fn add_body(body: &mut Vec<Code>, ui: &mut Ui, data: &mut Data) -> bool {
        let mut insert:Option<usize> = None;
        let mut remove:Option<usize> = None;
        let mut move_:Option<usize> = None;
        let mut add_item = false;
                
        ui.vertical(|ui| {
            
                        if match data.action {Action::Edit => false, _ => ui.label("add item").hovered()} {
                            if data.action == Action::Add || data.action == Action::Move {
                                add_code(ui, &mut data.drag_code);
                            }
                            
                            ui.ctx().input(|i| {
                                if i.pointer.any_click() {
                                    match data.action {
                                                Action::Add => {insert = Some(9999)},
                                                Action::Remove => {remove = Some(9999)},
                                                Action::Move => {
                                                    move_ = Some(9999);
                                                },
                                                Action::Edit => {},
                                            }
                                }
                            });
                        }
                        ui.vertical(|ui| {
                            
                            for (l,code) in body.iter_mut().enumerate() {
                                let mut render = false;
                                if ui.horizontal(|ui| {
                                    render = code.render(ui, data);
                                    if data.action == Action::Remove {
                                    if ui.small_button("x").clicked() {
                                        remove = Some(l);
                                    }
                                }
                                }).response.hovered() && render {
                                    if data.action == Action::Add {
                                add_code(ui, &mut data.drag_code);
                            }
                                    ui.ctx().input(|i| {
                                        if i.pointer.any_click() {
                                            match data.action {
                                                Action::Add => {insert = Some(l)},
                                                Action::Remove => {},
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
                    ui.group(|ui| {
                        if ui.label("add below").hovered() {
                            add_item = true;
                        };
                    });
                    
                    if let Some(l) = insert {
                        if body.len() == 0 && data.drag_code.is_some() {
                            body.push(data.drag_code.take().unwrap());
                    }else {
                        if data.drag_code.is_some() {
                            body.insert(match l {9999 => 0, l => l+1} , data.drag_code.take().unwrap());
                        }
                    }
                    }
                    if let Some(l) = remove {
                        println!("remove");
                        body.remove(l);
                    }
                    if let Some(l) = move_ {

                        println!("move");
                        data.action = Action::Add;
                        data.drag_code = body.get(match l {9999 => 0, l => l}).cloned();
                        body.remove(match l {9999 => 0, l => l});

                    }
        return add_item;
    }

    fn render(&mut self, ui: &mut Ui, data: &mut Data) -> bool {
        let mut add_below = true;
        match self {
            Code::Main { body } => {
                egui::Frame::group(&egui::Style::default()).fill(Color32::from_rgb(230, 210, 210)).rounding(10.0).show(ui, |ui|  {
                    ui.heading("Main");
                    add_below = Code::add_body( body, ui, data)
                });
            }
            Code::Function { name, body } => {
                egui::Frame::group(&egui::Style::default()).fill(Color32::from_rgb(210, 230, 210)).rounding(10.0).show(ui, |ui|  {
                    ui.add(egui::Label::new(egui::RichText::new(format!("{}", name)).heading()).wrap(false));
                    ui.horizontal(|ui| {
                        ui.label("Function: ");
                        ui.text_edit_singleline(name);
                    });
                    add_below = Code::add_body( body, ui, data);
                });
            }
            Code::Print { value } => {
                egui::Frame::group(&egui::Style::default()).fill(Color32::from_rgb(230, 210, 230)).rounding(10.0).show(ui, |ui|  {
                    ui.horizontal(|ui| {
                        ui.label("Print: ");
                        value.render(ui, data);
                    });
                });
            }
            Code::If { condition, body } => {
                egui::Frame::group(&egui::Style::default()).fill(Color32::from_rgb(250, 230, 210)).rounding(10.0).show(ui, |ui|  {
                    ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                    ui.label("If: ");
                    condition.render(ui, data);
                    });
                    add_below = Code::add_body( body, ui,  data);
                    });
                });
            }

            Code::While { condition, body } => {
                egui::Frame::group(&egui::Style::default()).fill(Color32::from_rgb(230, 250, 210)).rounding(10.0).show(ui, |ui|  {
                    ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                    ui.label("While : ");
                    condition.render(ui, data);
                    });
                    add_below = Code::add_body( body, ui, data);
                    });
                });
            }
            

            Code::AssignFloat { variable: float_variable, value } => {
                egui::Frame::group(&egui::Style::default()).fill(Color32::from_rgb(210, 210, 230)).rounding(10.0).show(ui, |ui|  {
                    ui.horizontal(|ui| {
                        ui.label("Assign Float: ");
                        let mut new_value:Option<FloatVariable> = None;
                        ui.menu_button(&float_variable.name, |ui| {
                            for (name, variable) in data.float_variables.iter_mut() {
                                if ui.small_button(name).clicked() {
                                    new_value = Some(variable.clone());
                                }
                            }
                        });
                if let Some(variable) = new_value {
                    *float_variable = variable;
                }
                        ui.label(" = ");
                        value.render(ui, &mut data.float_variables);
                    });
                });
            }
            Code::AssignBool { variable: bool_variable, value } => {
                egui::Frame::group(&egui::Style::default()).fill(Color32::from_rgb(210, 210, 230)).rounding(10.0).show(ui, |ui|  {
                    ui.horizontal(|ui| {
                        ui.label("Assign Bool: ");
                        let mut new_value:Option<BoolVariable> = None;
                        ui.menu_button(&bool_variable.name, |ui| {
                            for (name, variable) in data.bool_variables.iter_mut() {
                                if ui.small_button(name).clicked() {
                                    new_value = Some(variable.clone());
                                }
                            }
                        });
                if let Some(variable) = new_value {
                    *bool_variable = variable;
                }
                        ui.label(" = ");
                        value.render(ui, data);
                    });
                });
            }

            Code::AssignString { variable: string_variable, value } => {
                egui::Frame::group(&egui::Style::default()).fill(Color32::from_rgb(210, 210, 230)).rounding(10.0).show(ui, |ui|  {
                    ui.horizontal(|ui| {
                        ui.label("Assign String: ");
                        let mut new_value:Option<StringVariable> = None;
                        ui.menu_button(&string_variable.name, |ui| {
                            for (name, variable) in data.string_variables.iter_mut() {
                                if ui.small_button(name).clicked() {
                                    new_value = Some(variable.clone());
                                }
                            }
                        });
                if let Some(variable) = new_value {
                    *string_variable = variable;
                }
                        ui.label(" = ");
                        value.render(ui, data);
                    });
                });
            }

            Code::CallFunction { name } => {
                egui::Frame::group(&egui::Style::default()).fill(Color32::from_rgb(210, 210, 230)).rounding(10.0).show(ui, |ui|  {
                    ui.horizontal(|ui| {
                        ui.label("Call Function: ");
                        ui.menu_button(format!("{name}"), |ui| {
                            for func in data.function_names.iter() {
                                if ui.small_button(func).clicked() {
                                    *name = func.to_string();
                                }
                            }
                        });
                    });
                });
            }

            Code::Break => {
                egui::Frame::group(&egui::Style::default()).fill(Color32::from_rgb(255, 160, 160)).rounding(10.0).show(ui, |ui|  {
                    ui.label("Break");
                });
            }

            _ => {
                egui::Frame::group(&egui::Style::default()).fill(Color32::from_rgb(210, 210, 210)).rounding(10.0).show(ui, |ui|  {
                    ui.label("not yet added");
                });
            }
        }

        return add_below;

    }


    
}

#[derive(Clone, Debug,  Serialize, Deserialize)]
struct MathString {
    value: String,
}

#[derive(Clone, Debug,  Serialize, Deserialize)]
enum FloatOption {
    MathString(MathString),
    Variable(FloatVariable),
    Float(f32),
    Add(Box<FloatOption>, Box<FloatOption>),
    Subtract(Box<FloatOption>, Box<FloatOption>),
    Multiply(Box<FloatOption>, Box<FloatOption>),
    Divide(Box<FloatOption>, Box<FloatOption>),
    Abs(Box<FloatOption>),
    Negate(Box<FloatOption>),
    Sin(Box<FloatOption>),
    Cos(Box<FloatOption>),
    Tan(Box<FloatOption>),
    Random(Box<FloatOption>, Box<FloatOption>, bool),
    Round(Box<FloatOption>, Box<FloatOption>),
    Power(Box<FloatOption>, Box<FloatOption>),
    CercumferenceOfCircle(Box<FloatOption>),
    Pi,
    Tau,
    SquareRootOfTwo,
    GoldenRatio,
    EulersNumber,
    SpeedOfLight,
    PlankCostant,
    BigG,
    ElementaryCharge,

}

impl FloatOption {

    fn default_list() -> Vec<FloatOption> {
        vec![
            FloatOption::MathString(MathString { value: "1".to_string() }),
            FloatOption::Variable(FloatVariable { name: "1".to_string(), value: 1.0 }),
            FloatOption::Float(1.0),
            FloatOption::Add(Box::new(FloatOption::Float(1.0)), Box::new(FloatOption::Float(1.0))),
            FloatOption::Subtract(Box::new(FloatOption::Float(1.0)), Box::new(FloatOption::Float(1.0))),
            FloatOption::Multiply(Box::new(FloatOption::Float(1.0)), Box::new(FloatOption::Float(1.0))),
            FloatOption::Divide(Box::new(FloatOption::Float(1.0)), Box::new(FloatOption::Float(1.0))),
            FloatOption::Abs(Box::new(FloatOption::Float(1.0))),
            FloatOption::Negate(Box::new(FloatOption::Float(1.0))),
            FloatOption::Sin(Box::new(FloatOption::Float(1.0))),
            FloatOption::Cos(Box::new(FloatOption::Float(1.0))),
            FloatOption::Tan(Box::new(FloatOption::Float(1.0))),
            FloatOption::Random(Box::new(FloatOption::Float(1.0)), Box::new(FloatOption::Float(1.0)), true),
            FloatOption::Round(Box::new(FloatOption::Float(1.0)), Box::new(FloatOption::Float(0.0))),
            FloatOption::Power(Box::new(FloatOption::Float(1.0)), Box::new(FloatOption::Float(1.0))),
            FloatOption::Pi,
            FloatOption::Tau,
            FloatOption::SquareRootOfTwo,
            FloatOption::GoldenRatio,
            FloatOption::SpeedOfLight,
            FloatOption::PlankCostant,
            FloatOption::BigG,
            FloatOption::ElementaryCharge,

        ]
    }

    fn name(&self) -> String{
        match self {
            FloatOption::MathString(_math_string) => "Math String",
            FloatOption::Variable(_float_variable) => "Variable",
            FloatOption::Add(_float1, _float2) => "Add",
            FloatOption::Subtract(_float1, _float2) => "Subtract",
            FloatOption::Multiply(_float1, _float2) => "Multiply",
            FloatOption::Divide(_float1, _float2) => "Divide",
            FloatOption::Float(_float) => "Float",
            FloatOption::Abs(_float) => "Abs",
            FloatOption::Negate(_float) => "Negate",
            FloatOption::Sin(_float) => "Sin",
            FloatOption::Cos(_float) => "Cos",
            FloatOption::Tan(_float) => "Tan",
            FloatOption::Random(_float1, _float2, _) => "Random",
            FloatOption::Round(_float, _float2) => "Round",
            FloatOption::Power(_float1, _float2) => "Power (+/- integer powers)",
            FloatOption::CercumferenceOfCircle(_float) => "Cercumference of Circle",
            FloatOption::Pi => "Pi",
            FloatOption::Tau => "Tau",
            FloatOption::SquareRootOfTwo => "Square Root of Two",
            FloatOption::GoldenRatio => "Golden Ratio",
            FloatOption::EulersNumber => "Euler's Number (e)",
            FloatOption::SpeedOfLight => "Speed of Light",
            FloatOption::PlankCostant => "Plank Constant",
            FloatOption::BigG => "Big G",
            FloatOption::ElementaryCharge => "Elementary Charge",

            
            
        }.to_string()
    }

    fn render(&mut self, ui: &mut Ui, float_variables: &mut HashMap<String, FloatVariable>) {
        egui::Frame::group(&egui::Style::default()).fill(Color32::from_rgb(250, 210, 230)).rounding(10.0).show(ui, |ui|  {
        ui.horizontal_centered(|ui| {
            ui.menu_button(self.name(), |ui| {
                for option in FloatOption::default_list() {
                if ui.small_button(&option.name()).clicked() {
                    *self = option;
                }
            }
    });
    match self {
        FloatOption::Pi
            |FloatOption::Tau
            |FloatOption::SquareRootOfTwo
            |FloatOption::GoldenRatio
            |FloatOption::EulersNumber
            |FloatOption::SpeedOfLight
            |FloatOption::PlankCostant
            |FloatOption::BigG
            |FloatOption::ElementaryCharge => {return}
        _ => {}
    }
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
            FloatOption::Add(float1, float2) => {
                float1.render(ui, float_variables);
                ui.label("+");
                float2.render(ui, float_variables);
            }
            FloatOption::Subtract(float1, float2) => {
                float1.render(ui, float_variables);
                ui.label("-");
                float2.render(ui, float_variables);
            }
            FloatOption::Multiply(float1, float2) => {
                float1.render(ui, float_variables);
                ui.label("x");
                float2.render(ui, float_variables);
            }
            FloatOption::Divide(float1, float2) => {
                
                ui.vertical(|ui| {
                    float1.render(ui, float_variables);
                    ui.label("----------");
                    float2.render(ui, float_variables);
                });

            }
            FloatOption::Abs(float) => {
                ui.label("|");
                float.render(ui, float_variables);
                ui.label("|");
            }
            FloatOption::Negate(float) => {
                ui.label("-");
                float.render(ui, float_variables);
            }
            FloatOption::Sin(float) => {
                float.render(ui, float_variables);
            }
            FloatOption::Cos(float) => {
                ui.label("(");
                float.render(ui, float_variables);
                ui.label(")");
            }
            FloatOption::Tan(float) => {
                ui.label("(");
                float.render(ui, float_variables);
                ui.label(")");
            }
            FloatOption::Random(float1, float2, rounded) => {
                ui.label("between ");
                float1.render(ui, float_variables);
                ui.label(" and ");
                float2.render(ui, float_variables);
                ui.checkbox(rounded, "rounded");
            }
            FloatOption::Round(num, dp) => {
                ui.label("round ");
                num.render(ui, float_variables);
                ui.label("to ");
                dp.render(ui, float_variables);
                ui.label("dp (not working)");
            }
            FloatOption::Power(num, power) => {
                num.render(ui, float_variables);
                ui.label("to the power of");
                power.render(ui, float_variables);
            }
            FloatOption::Pi
            |FloatOption::Tau
            |FloatOption::SquareRootOfTwo
            |FloatOption::GoldenRatio
            |FloatOption::EulersNumber
            |FloatOption::SpeedOfLight
            |FloatOption::PlankCostant
            |FloatOption::BigG
            |FloatOption::ElementaryCharge

             => {
                
            }

            FloatOption::CercumferenceOfCircle(radius) => {
                ui.label("cercumference of circle with radius ");
                radius.render(ui, float_variables);
            }

        }
        });
    });

    }
    
    // fn string(&self) -> String {
    //     return "".to_owned();
    // //     return match self {
    // //         FloatOption::MathString(math_string) => math_string.value.clone(),
    // //         FloatOption::Variable(float_variable) => float_variable.name.clone(),
    // //         FloatOption::Float(float) => float.to_string(),
    // // }
    
    // }
    // fn float(&self) -> f32 {
    //     return match self {
    //         FloatOption::MathString(math_string) => math_string.value.parse::<f32>().unwrap_or(0.0),
    //         FloatOption::Variable(float_variable) => float_variable.value,
    //         FloatOption::Float(float) => *float,
    //     }
    // }

    fn output(&self) -> String {
        return match self {
            FloatOption::MathString(math_string) => math_string.value.clone(),
            FloatOption::Variable(float_variable) => float_variable.name.clone(),
            FloatOption::Float(float) => float.to_string(),
            FloatOption::Add(float1, float2) => format!("({}+{})", float1.output(), float2.output()),
            FloatOption::Subtract(float1, float2) => format!("({}-{})", float1.output(), float2.output()),
            FloatOption::Multiply(float1, float2) => format!("({}*{})", float1.output(), float2.output()),
            FloatOption::Divide(float1, float2) => format!("({}/{})", float1.output(), float2.output()),
            FloatOption::Abs(float) => format!("Abs({})", float.output()),
            FloatOption::Negate(float) => format!("(-1*{})", float.output()),
            FloatOption::Sin(float) => format!("Sin({})", float.output()),
            FloatOption::Cos(float) => format!("Cos({})", float.output()),
            FloatOption::Tan(float) => format!("Tan({})", float.output()),
            FloatOption::Random(float1, float2, rounded) => 
            match rounded {
                true => format!("(Int({}+(Ran#)*({}-{})+0.5))", float1.output(),float2.output(), float1.output()),
                false =>format!("({}+(Ran#)*({}-{}))",float1.output(), float2.output(), float1.output())
            }
            FloatOption::Round(num, dp) => format!("Int({}+0.5)/", num.output()),
            FloatOption::Power(num, power) => format!("({}^{})", num.output(), power.output()),
            FloatOption::Pi => "(3+670889731/4738167652)".to_owned(),
            FloatOption::Tau => "(6+2*670889731/4738167652)".to_owned(),
            FloatOption::SquareRootOfTwo => "(99/70)".to_owned(),
            FloatOption::BigG => "(6.674/100000000)".to_owned(),
            FloatOption::EulersNumber => "(2.7182818284)".to_owned(),
            FloatOption::GoldenRatio => "Max(2*sin(54), 2*sin((54/180)*(3+670889731/4738167652)))".to_owned(),
            FloatOption::SpeedOfLight => "(299792458)".to_owned(),
            FloatOption::PlankCostant => "(6.62607015/100000000)".to_owned(),
            FloatOption::ElementaryCharge => "(1.602176634/100000000)".to_owned(),
            FloatOption::CercumferenceOfCircle(radius) => format!("(2*{}*{})", radius.output(), FloatOption::Pi.output()),

        }
    }


}


// 2->A
// -4->N
// abs(N)-> Dim List 5
// fill(A, List 5)
// (-N/(abs(N))/2+0.5)/(Prod List 5)+(Prod List 5)*(N/(abs(N))/2+0.5)

#[derive(Clone, Debug,  Serialize, Deserialize)]
struct BoolVariable {
    name: String,
    value: bool,
}

impl BoolVariable {
    fn output(&self) -> String {
        return format!("({} = 1)", self.name);
    }
}

#[derive(Clone, Debug,  Serialize, Deserialize)]
enum BoolOption {
    Variable(BoolVariable),
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
    fn render(&mut self, ui: &mut Ui, data: &mut Data) {
        egui::Frame::group(&egui::Style::default()).fill(Color32::from_rgb(255, 250, 160)).rounding(10.0).show(ui, |ui|  {
        ui.horizontal(|ui| {
        match self {
            BoolOption::Equal(a, _) => a.render(ui, &mut data.float_variables),
            BoolOption::NotEqual(a, _) => a.render(ui,&mut data.float_variables),
            BoolOption::Less(a, _) => a.render(ui,&mut data.float_variables),
            BoolOption::LessEqual(a, _) => a.render(ui,&mut data.float_variables),
            BoolOption::Greater(a, _) => a.render(ui,&mut data.float_variables),
            BoolOption::GreaterEqual(a, _) => a.render(ui,&mut data.float_variables),
            _ => {}
        }
        ui.menu_button(match self {
            BoolOption::Variable(_bool_variable) => "Variable",
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
                    for (name, bool_variable) in data.bool_variables.iter_mut() {
                        if ui.small_button(name).clicked() {
                            new_value = Some(bool_variable.clone());
                        }
                    }
                });
                if let Some(new_value) = new_value {
                    *bool_variable = new_value;
                }
            }
            BoolOption::Equal(_, b) => b.render(ui, &mut data.float_variables),
            BoolOption::NotEqual(_, b) => b.render(ui, &mut data.float_variables),
            BoolOption::Less(_, b) => b.render(ui, &mut data.float_variables),
            BoolOption::LessEqual(_, b) => b.render(ui, &mut data.float_variables),
            BoolOption::Greater(_, b) => b.render(ui, &mut data.float_variables),
            BoolOption::GreaterEqual(_, b) => b.render(ui, &mut data.float_variables),
            _ => {},
        }
        });
    });
    }

    fn output(&self) -> String {
        match self {
            BoolOption::False => "0".to_owned(),
            BoolOption::True => "1".to_owned(),
            BoolOption::Variable(bool_variable) => format!("({})", bool_variable.name),
            BoolOption::Equal(a, b) => format!("({}={})", a.output(), b.output()),
            BoolOption::NotEqual(a, b) => format!("({}<>{})", a.output(), b.output()),
            BoolOption::Less(a, b) => format!("({}<{})", a.output(), b.output()),
            BoolOption::LessEqual(a, b) => format!("({}<={})", a.output(), b.output()),
            BoolOption::Greater(a, b) => format!("({}>{})", a.output(), b.output()),
            BoolOption::GreaterEqual(a, b) => format!("({}>={})", a.output(), b.output()),

        }
    }
}
#[derive(Clone, Debug,  Serialize, Deserialize)]
struct FloatVariable {
    name: String,
    value: f32,
}

impl FloatVariable {
    fn output(&self) -> String {
        return self.name.clone();
    }
}

#[derive(Clone, Debug,  Serialize, Deserialize)]
struct StringVariable {
    name: String,
    value: String,
}

impl StringVariable {
    fn output(&self) -> String {
        return self.name.clone();
    }
}


#[derive(Clone, Debug,  Serialize, Deserialize)]
enum StringOption {
    StringConstant(String),
    // Float(FloatOption),
    StringVariable(StringVariable),
}

impl StringOption {
    fn render(&mut self, ui: &mut Ui, data: &mut Data) {
        ui.horizontal(|ui| {
        ui.menu_button(match self {
            // StringOption::Float(_float) => "Float",
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
            // if ui.small_button("float").clicked() {
            //     *self = StringOption::Float(FloatOption::Float(0.0));
            // }
    });
        match self {
            StringOption::StringConstant(string) => {
                ui.text_edit_singleline(string);
            }
            StringOption::StringVariable(string_variable) => {
                let mut new_value:Option<StringVariable> = None;
                ui.menu_button(&string_variable.name, |ui| {
                    for (name, string_variable) in data.string_variables.iter_mut() {
                        if ui.small_button(name).clicked() {
                            new_value = Some(string_variable.clone());
                        }
                    }
                });
                if let Some(new_value) = new_value {
                    *string_variable = new_value;
                }
            }
            // StringOption::Float(float_option) => {
            //     float_option.render(ui, &mut HashMap::new());
            // }
        }
        });
    }


    fn output(&self) -> String {
        return format!("\"{}\"", match self {
            StringOption::StringConstant(string) => string.clone(),
            StringOption::StringVariable(string_variable) => string_variable.name.clone(),
            // StringOption::Float(float_option) => float_option.output(),
        });
    }
}
#[derive(Clone, Debug,  Serialize, Deserialize)]
struct Program {
    name: String,
    code: Vec<Code>,
}

impl Default for Program {
    fn default() -> Self {
        Program {
            name: "Unnamed Program".to_string(),
            code: vec![Code::Main { body: vec![
                Code::Print { value: StringOption::StringConstant("Hello World".to_owned()) }
            ] }],
        }
    }

    
}

fn output_block( block: &Vec<Code>, label: &mut usize, fn_calls:&mut HashMap<String,Vec<usize>>) -> Vec<String> {
    let mut out = vec![];
    for (i,code) in block.iter().enumerate() {
        match code {
            Code::Main { body } => {
                out.append(&mut output_block(body, label, fn_calls));
            },
            Code::Print { value } => {
                out.push(value.output());
            },
            Code::AssignBool { variable, value } => {
                out.push(format!("{}->{}", value.output(), variable.output()));
            },
            Code::AssignFloat { variable, value } => {
                out.push(format!("{}->{}", value.output(), variable.output()));
            },
            Code::AssignString { variable, value } => {
                out.push(format!("{}->{}", value.output(), variable.output()));
            },
            Code::If { condition, body } => {
                out.push(format!("If {} \n Then", condition.output()));
                out.append(&mut output_block(body, label, fn_calls));
                out.push("IfEnd".to_owned());
            }
            
            Code::Function { name, body } => {
                // let mut function_body = vec!["".to_owned()];
                // function_body.append(&mut output_block(body));
            }
            Code::Break => {
                out.push("Break".to_owned());
            }

            Code::While { condition, body } => {
                out.push(format!("While ({})", condition.output()));
                out.append(&mut output_block(body, label, fn_calls));
                out.push("WhileEnd".to_owned());
            }
            Code::CallFunction { name } => {
                out.push(format!("'call {}", name));
                out.push("List 4[1]->K".to_owned());
                out.push("For 2->I to 200 Step 1".to_owned());
                out.push("K->G".to_owned());
                out.push("List 4[I]->K".to_owned());
                out.push("G -> List 4[I]".to_owned());
                out.push("List 4[I]=0 => 200->I".to_owned());
                out.push("Next".to_owned());
                out.push(format!("{label} -> List 4[1]"));
                out.push(format!("Goto $[fn_line:{}]", name));
                out.push(format!("Lbl {label}"));
                fn_calls.entry(name.clone()).or_insert(vec![]).push(*label);
                *label += 1;
            }
        }
    }
    return out;
}

impl Program {
    fn from_string(string: String) -> Self {
        let mut p =Program::default();
        p.name = string;
        return p;
    }

    fn output(&self, data: &mut Data) -> String {
        let mut out = vec![];
        let mut fn_calls: HashMap<String, Vec<usize>> = HashMap::new();


        out.push(format!("{}->Dim List 1", data.string_variables.len()));
        let var_string_array: Vec<String> = data.string_variables.keys().map(|x|x.to_owned()).collect();
        for (i,k) in var_string_array.iter().enumerate() {
            out.push(format!("'\"{}\"->List 1[{}]", data.string_variables.get(k).unwrap().value, i+1));
        }

        out.push(format!("{}->Dim List 2", data.float_variables.len()));
        let var_float_array: Vec<String> = data.float_variables.keys().map(|x|x.to_owned()).collect();
        for (i,k) in var_float_array.iter().enumerate() {
            out.push(format!("{}->List 2[{}]", data.float_variables.get(k).unwrap().value, i+1));
        }

        out.push(format!("{}->Dim List 3", data.bool_variables.len()));
        let var_bool_array: Vec<String> = data.bool_variables.keys().map(|x|x.to_owned()).collect();
        for (i,k) in var_bool_array.iter().enumerate() {
            out.push(format!("{}->List 3[{}]", match data.bool_variables.get(k).unwrap().value {
                true => 1,
                false => 0,
            } , i+1));
        }

        out.push(format!("{}->Dim List 4", 255));

        let mut label = 1;

        let mut functions:Vec<(String, usize)> = vec![];

        for i in self.code.iter() {
            match i {
                Code::Main { body } => {
                    out.append(&mut output_block(body, &mut label, &mut fn_calls));
                },
                Code::Function { body, name } => {
                    let replace = out.len();
                    out.push(format!("'replace me"));
                    out.push(format!("Lbl {label}"));
                    functions.push((name.clone(), label));
                    label += 1;
                    out.append(&mut output_block(body, &mut (label), &mut fn_calls));
                    out.push("List 4[1] -> T".to_owned());
                    out.push("For 2->I to 200 Step 1".to_owned());
                    out.push("List 4[I] -> List 4[I-1]".to_owned());
                    out.push("List 4[I]=0 => 200->I".to_owned());
                    out.push("Next".to_owned());
                    out.push(format!("' $[return_from{name}]"));
                    out.push(format!("Lbl {label}"));
                    out[replace] = format!("Goto {}", label);
                    label += 1;
                }
                _ => {}
            }
        }

        


        let mut final_string = out.join("\n");
        for i in 0..var_string_array.len() {
            final_string = final_string.replace(&var_string_array[i], &format!("List 1[{}]", i+1));
        }
        for i in 0..var_float_array.len() {
            final_string = final_string.replace(&var_float_array[i], &format!("List 2[{}]", i+1));
        }
        for i in 0..var_bool_array.len() {
            final_string = final_string.replace(&var_bool_array[i], &format!("(List 3[{}]=1)", i+1));
        }

        for i in functions.iter() {
            final_string = final_string.replace(&format!("$[fn_line:{}]", i.0), &format!("{}", i.1));
        }

        for i in fn_calls.iter() {
            final_string = final_string.replace(&format!("' $[return_from{}]", i.0), 
            &format!("{}", i.1.iter().map(|x|format!("{x}=T => Goto {x}")).collect::<Vec<String>>().join("\n"))
        );
        }




        return final_string;
    }

    fn render(&mut self, egui_ctx: &egui::Context, data: &mut Data) {
        egui::CentralPanel::default()
        .show(egui_ctx, |ui| {
            egui::ScrollArea::both()
            .always_show_scroll(false)
            .max_width(f32::INFINITY)
            .max_height(f32::INFINITY)
            
            .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
            let mut remove_fn:Option<usize> = None;
            for (i,function) in self.code.iter_mut().enumerate() {
                ui.vertical(|ui| {
                    function.render(ui, data );
                    match function {
                    Code::Function { name, body:_ } => {
                        if data.action == Action::Remove {
                            if ui.button("Del Function").clicked() {
                                remove_fn = Some(i);
                                data.function_names = vec![];
                            }
                        }
                        if data.function_names.len() <= (i-1) {
                            data.function_names.push(name.to_string());
                        }
                        else if data.function_names[i-1] != name.to_string() {
                            data.function_names[i-1] = name.to_string();
                        }
                    }
                    
                    _ => {}
                }
            });
            }
            if let Some(i) = remove_fn {
                self.code.remove(i);
            }
            });
            if ui.button("Add Function +").clicked() {
                self.code.push(Code::Function { name: "Unnamed Function".to_owned(), body: vec![] });
                data.function_names=vec![];
                
            }
            ui.add_sized(ui.available_size(), Label::new(""));
            });
        });

    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Data {
    drag_code: Option<Code>, 
    action: Action,
    string_variables: HashMap<String, StringVariable>,
    float_variables: HashMap<String, FloatVariable>, 
    bool_variables: HashMap<String, BoolVariable>,
    function_names: Vec<String>,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            drag_code: None,
            action: Action::Add,
            string_variables: HashMap::new(),
            float_variables: HashMap::new(),
            bool_variables: HashMap::new(),
            function_names: vec![],
        }
    }
}

#[macroquad::main("egui with macroquad")]
async fn main() {

    let programs:HashMap<String, Program> = HashMap::new();

    let paths = match fs::read_dir("./Programs") {
        Ok(p) => p.into_iter().collect(),
        Err(e) => vec![]
    };



    
    egui_macroquad::ui(|egui_ctx| {
        egui_ctx.set_visuals(egui::Visuals::light());
    });

    let mut main_menu = true;
    let mut new_program_name = "Unnamed Program".to_owned();

    let mut program = Program::default();

    let mut now = Instant::now();


    let mut data = Data {
        drag_code: None,
        action: Action::Add,
        string_variables: HashMap::new(),
        float_variables: HashMap::new(),
        bool_variables: HashMap::new(),
        function_names: vec![],
    };


    let mut new_bool_variable_name = "Unnamed Variable".to_owned();
    let mut new_string_variable_name = "Unnamed Variable".to_owned();
    let mut new_float_variable_name = "Unnamed Variable".to_owned();


    let mut compiled_text = String::new();


    
    loop {
        clear_background(WHITE);

        if !main_menu && now.elapsed().as_secs() >= 2 {
            now = Instant::now();
            let serialized = match serde_json::to_string(&(&program, &data)) {
                Ok(s) => s,
                Err(e) => "".to_owned(),
            };

            match File::create(format!("./Programs/{}.json", program.name)) {
                Ok(mut f) => {
                    match f.write_all(serialized.as_bytes()) {
                        Ok(_) => {},
                        Err(e) => panic!("Error writing to file: {}", e),
            }
                },
                Err(e) => {println!("Error creating file: {}", e)},
            };

        }

        egui_macroquad::ui(|egui_ctx| {

            if main_menu {

                egui::CentralPanel::default().show(egui_ctx, |ui| {
                    
                    egui::ScrollArea::vertical()
                    
                    .max_height(ui.available_height())
                    .max_width(ui.available_width())
                    .show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(20.0);
                            ui.heading("Main Menu");
                            ui.add_space(20.0);
                            egui::scroll_area::ScrollArea::vertical().show(ui, |ui|{
                                for path in &paths {
                                let name = match path {
                                    Ok(p) => match p.file_name().into_string() {
                                        Ok(s) => s,
                                        Err(e) => format!("error {e:?}"),
                                    },
                                    Err(e) => format!("error {e}"),
                                
                                };
                                if ui.button(&name).clicked() {
                                    main_menu = false;
                                    let string = match fs::read_to_string(format!("./Programs/{}", name)) {
                                        Ok(s) => s,
                                        Err(e) => format!("reading file  {e}"),
                                    };
                                    let new:(Program, Data) = 
                                    match serde_json::from_str(
                                        &string) {
                                            Ok(p) => p,
                                            Err(e) => {println!("error {e}");println!("{}",string);( Program::default(), Data::default())},
                                        };
                                    program = new.0;
                                    data=new.1
                                    
                                };
                            }
                        });
                            ui.text_edit_singleline(&mut new_program_name);
                            if ui.button(" Add +").clicked() {
                                main_menu = false;
                                program = Program::from_string(new_program_name.clone());
                            }
                            });
                    });
                });


            }else {

            egui::TopBottomPanel::top("top")
            .resizable(false).show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(&program.name);
                    if ui.button("Main Menu").clicked() {
                        main_menu = true;
                    }
                });
            
            });

            egui::SidePanel::left("my_side_panel").show(egui_ctx, |ui| {
                ui.radio_value(&mut data.action, Action::Edit, "Edit");
                ui.radio_value(&mut data.action, Action::Add, "Add");
                ui.radio_value(&mut data.action, Action::Remove, "Remove");
                ui.radio_value(&mut data.action, Action::Move, "Move");

                ui.separator();
                let mut remove:Option<String> = None;

                ui.label("String Variables:");
                for (name, string_variable) in data.string_variables.iter_mut() {
                    ui.horizontal(|ui| {
                        ui.label(name);
                        ui.text_edit_singleline(&mut string_variable.value);
                        if data.action == Action::Remove {
                        if ui.button("X").on_hover_text("Remove Variable").clicked() {
                            remove = Some(name.clone());
                            data.function_names = vec![];
                        }
                    }
                    });
                }
                if let Some(name) = remove {
                    data.string_variables.remove(&name);
                }
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut new_string_variable_name);
                    new_string_variable_name = new_string_variable_name.to_lowercase()
                    .replace(" ", "-")
                    .to_owned();
                
                    if ui.button("Add String Variable").clicked() {
                        if !new_string_variable_name.starts_with("$[") {
                    new_string_variable_name = format!("$[{}", new_string_variable_name);
                }
                if !new_string_variable_name.ends_with("]") {
                    new_string_variable_name = format!("{}]", new_string_variable_name);
                }
                        data.string_variables.insert(new_string_variable_name.clone(), StringVariable {
                            name: new_string_variable_name.clone(),
                            value: "".to_owned(),
                        });
                    }
                });
                let mut remove:Option<String> = None;
                ui.separator();
                ui.label("Float Variables:");
                for (name, float_variable) in data.float_variables.iter_mut() {
                    ui.horizontal(|ui| {
                        ui.label(name);
                        ui.add(egui::DragValue::new(&mut float_variable.value).max_decimals_opt(Some(7)));
                        if ui.button("X").on_hover_text("Remove Variable").clicked() {
                            remove = Some(name.clone());
                        }
                    });
                }
                if let Some(name) = remove {
                    data.float_variables.remove(&name);
                }
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut new_float_variable_name);
                    new_float_variable_name = new_float_variable_name.to_lowercase()
                    .replace(" ", "-")
                    .to_owned();
                    if ui.button("Add Float Variable").clicked() {
                        if !new_float_variable_name.starts_with("$[") {
                    new_float_variable_name = format!("$[{}", new_float_variable_name);
                }
                if !new_float_variable_name.ends_with("]") {
                    new_float_variable_name = format!("{}]", new_float_variable_name);
                }

                        data.float_variables.insert(new_float_variable_name.clone(), FloatVariable {
                            name: new_float_variable_name.clone(),
                            value: 0.0,
                        });
                    }
                });

                ui.separator();
                let mut remove:Option<String> = None;
                ui.label("Bool Variables:");
                for (name, bool_variable) in data.bool_variables.iter_mut() {
                    ui.horizontal(|ui| {
                        ui.label(name);
                        ui.checkbox(&mut bool_variable.value, "");
                        if ui.button("X").on_hover_text("Remove Variable").clicked() {
                            remove = Some(name.clone());
                        }
                    });
                }
                if let Some(name) = remove {
                    data.bool_variables.remove(&name);
                }
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut new_bool_variable_name);
                    new_bool_variable_name = new_bool_variable_name.to_lowercase()
                    .replace(" ", "-")
                    .to_owned();

                    if ui.button("Add Bool Variable").clicked() {
                        if !new_bool_variable_name.starts_with("$[") {
                    new_bool_variable_name = format!("$[{}", new_bool_variable_name);
                }
                if !new_bool_variable_name.ends_with("]") {
                    new_bool_variable_name = format!("{}]", new_bool_variable_name);
                }
                        data.bool_variables.insert(new_bool_variable_name.clone(), BoolVariable {
                            name: new_bool_variable_name.clone(),
                            value: true,
                        });
                    }
                });

                ui.separator();
                if data.action == Action::Add {
                    ui.heading("Add Code Block").on_hover_text("You can edit your blocks of code here before you add them to the program");
                    add_code(ui,&mut data.drag_code);
                    ui.separator();
                    ui.heading("Add Code Block");
                }
                for c in [
                    Code::Print { value: StringOption::StringConstant("Hello World".to_owned()) },
                    Code::AssignBool { variable: BoolVariable { name: "Example Boolean Variable".to_owned(), value: true }, value: BoolOption::True },
                    Code::AssignFloat { variable: FloatVariable { name: "Example Float Variable".to_owned(), value: 0.0 }, value: FloatOption::Float(1.0) },
                    Code::AssignString { variable: StringVariable { name: "Example String Variable".to_owned(), value: "Default String".to_owned() }, value: StringOption::StringConstant("Hello World".to_owned()) },
                    Code::If { condition: BoolOption::True, body: vec![] },
                    Code::While { condition: BoolOption::False, body: vec![] },
                    Code::CallFunction { name: "Example Function".to_owned() },
                    Code::Break,
                ].iter_mut() {
                    let rect = ui.horizontal(|ui| {
                        c.render(ui, &mut data);
                    }).response.rect;
                    if ui.put(rect, egui::Button::new("").fill(egui::Color32::TRANSPARENT).frame(false)).clicked() {
                        data. drag_code = Some(c.clone());
                    };
                }
            });

            egui::SidePanel::right("output")
            
            .show(egui_ctx, |ui| {
                egui::scroll_area::ScrollArea::vertical().show(ui, |ui| {

                    ui.horizontal(|ui| {
                        ui.label("Output");
                    if ui.button("Recompile").clicked() {
                        compiled_text = program.output(&mut data);
                    }
                    if ui.button("Copy To Clipboard").clicked() || ui.input(|i| {i.key_pressed(egui::Key::C)}) {
                        compiled_text = program.output(&mut data);
                        egui_ctx.output_mut(|o| {
                            o.copied_text = compiled_text.clone();
                        }
                        );
                    }
                });
                ui.separator();
                
                egui::TextEdit::multiline(&mut compiled_text)
                .code_editor()        
                        
                .show(ui);
                // ui.label(&compiled_text);
            });

            });

            // egui::Area::new("Drag").order(egui::Order::Tooltip).interactable(false)
            // .fixed_pos(match egui_ctx.pointer_interact_pos() {
            //     Some(pos) => pos,
            //     None => egui::Pos2::new(0.0, 0.0),
            // })
            // .show(egui_ctx, |ui| {
            //     match &mut data.drag_code {
            //         Some(code) => {
            //             // code.render(ui, &mut None);
            //         }
            //         None => {
            //         }
            //     }
            // });

            program.render(egui_ctx, &mut data);
        }
        });

        // Draw things before egui

        

        egui_macroquad::draw();
        
        // Draw things after egui

        next_frame().await;
    }
}