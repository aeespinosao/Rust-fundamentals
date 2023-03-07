fn variables() {
    // Inmutable variables
    let x = 8;
    println!("El valor de la variable es: {}", x);

    let y:u8;
    y=10;
    println!("El valor de la variable es: {}", y);
    
    // Mutable variables
    let mut z = 5;
    println!("El valor de la variable es: {}", z);
    z=10;
    println!("El valor de la variable es: {}", z);

    // Shadow win
    let w = 5;
    println!("El valor de la variable es: {}", w);
    let w = w+16;
    println!("El valor de la variable es: {}", w);

    // Constants
    const SPACES:i32 = 3;
    println!("Cantidad de espacios {}", SPACES);
}

fn typing() {
    // Integer
    let entero: i8 = 23;
    let entero2: u8 = 10;
    let entero3: i8 = -110;

    // Integer literals
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1100;

    // Float
    let float1 = 5.0;
    let float2:f32 = 12.432;
    
    // Boolean
    let verdadero = true;
    let falso: bool = false;

    // Char
    let caracter = 'c';

    // Compound types
    let tupla = ('h', 23, -45, 0.43);
    let tupla2: (char, i32, u8) = ('c', -3, 8);
    let (x,y,z) = tupla2;

    let array = [0,1,2,3];
    let array2: [i32;4] = [0,2,3,6];

    // Strings
    let nombre:&'static str = "andres";

    let apellido: String = "espinosa".to_string();

    let mut direccion = String::new();
    direccion = "casa".to_string();

    
}

fn select_number(number:i8) {
    println!("El numero seleccionado es {}", number);
}

fn return_number(number:i8) -> i8 {
    number
}

fn by_ref(number: &i8) -> i8 {
    *number + 3
}

fn structs() {
    struct Usuario {
        nombre: String,
        edad: i8,
    }

    let user = Usuario {
        nombre: String::from("andres"),
        edad: 28
    };

    println!("nombre {} edad {}", user.nombre, user.edad);
    
    let user2 = Usuario {
        nombre: "juan".to_string(),
        ..user
    };
    println!("nombre {} edad {}", user2.nombre, user2.edad);

    struct Point(i8,i8,i8);

    let point = Point(1,2,3);
}

fn enums() {
    enum UserRoles {
        BASIC,
        ADMIN,
    }

    let role = UserRoles::ADMIN;

    enum Website {
        URL(String),
        INSTAGRAM(String),
        LINKEDIN(String)
    }

    let web = Website::INSTAGRAM(String::from("https://ig.com"));

    enum Option<T> {
        Some(T),
        None
    }

    let nombre: Option<String> = None;
    nombre = Some("Andres".toString());
}

fn traits() {
    struct Humano;
    struct Gato;

    trait Hablar {
        fn hablar(&self) -> String;
    }

    impl Hablar for Humano {
        fn hablar(&self) -> String {
            "Hola".to_string()
        }
    }

    impl Hablar for Gato {
        fn hablar(&self) -> String {
            "MIAU".to_string()
        }
    }
}

fn iterator() {
    let s = [1,2,3];

    for x in s.iter() {
        println!("{}", x+1);
    }
}

fn closures() {
    let sum = |n1,n2| {
        n1+n2;
    }

    println!("{}", sum(1,2));
}

fn loops() {
    let mut counter = 0;
    let result = loop {
        if counter == 10 {
            break counter;
        }

        counter += 1;
    }
    println!("{}", result);

    while counter>0 {
        println!("{}", counter);
        counter -= 1;
    }
    
    let arreglo = [0,1,2,3,4];
    
    for element in arreglo.iter() {
        println!("{}", element);
    }
}

fn if_let() {
    let edad: Option<i32> = Some(20);

    if let Some(value) = edad {
        println!("{}", value);
    } else {
        println!("No valido");

    }

    let mut mensajes = Some(100);

    while let  Some(value) = mensajes {
        if value > 0 {
            println!("{} no leidos", value);
            mensajes = Some(value-1);
        } else {
            println!("No hay mensajes");
            mensajes = None;
        }
    }
}

fn main(){
    variables();
    typing();
    select_number(10);
    println!("el numero retornado es {}", return_number(14));
    let mut z =  14;
    println!("el numero por ref es {}", by_ref(&z));
    
    structs();
    
}