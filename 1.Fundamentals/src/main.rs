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

fn main(){
    variables();
    typing();
    select_number(10);
    println!("el numero retornado es {}", return_number(14));
    let mut z =  14;
    println!("el numero por ref es {}", by_ref(&z));
    
    structs();
    
}