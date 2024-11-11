use std::io;

// Definimos la estructura de la Moto
struct Moto {
    id_cola: usize,
    marca: String,
    cilindraje: u32,
    modelo: u32,
    precio: f32,
    placa: String,
}

impl Moto {
    fn new(id_cola: usize, placa: String, marca: String, cilindraje: u32, modelo: u32, precio: f32) -> Self {
        Moto {
            id_cola,
            placa,
            marca,
            cilindraje,
            modelo,
            precio,
        }
    }

    // Método para mostrar los datos de la moto sin la placa
    fn mostrar(&self) {
        println!("ID Cola: {}", self.id_cola);
        println!("Marca: {}", self.marca);
        println!("Cilindraje: {}cc", self.cilindraje);
        println!("Modelo: {}", self.modelo);
        println!("Precio: ${}", self.precio);
    }
}

// Función para agregar una moto a la cola
fn meter(cola: &mut Vec<Moto>) {
    let id = cola.len() + 1;

    println!("Ingrese los datos de la moto:");

    // Verificar si la placa ya existe antes de agregar la moto
    let placa = loop {
        let placa = leer_texto("Ingrese la placa: ");
        if cola.iter().any(|moto| moto.placa == placa) {
            println!("Error: Ya existe una moto registrada con la placa {}. Ingrese una placa diferente.", placa);
        } else {
            break placa;
        }
    };

    let marca = leer_texto("Ingrese la marca: ");
    let cilindraje = leer_entero("Ingrese el cilindraje (cc): ");
    let modelo = loop {
        let modelo = leer_entero("Ingrese el modelo (año): ");
        if modelo >= 1900 && modelo <= 2025 {
            break modelo;
        } else {
            println!("Error: El modelo debe estar entre 1900 y 2025.");
        }
    };
    let precio = loop {
        let precio = leer_flotante("Ingrese el precio en USD: $");
        if precio >= 0.0 {
            break precio;
        } else {
            println!("Error: El precio no puede ser negativo.");
        }
    };

    let moto = Moto::new(id, placa, marca, cilindraje, modelo, precio);
    cola.push(moto);
}

// Función para sacar una moto de la cola y almacenarla en eliminados
fn sacar(cola: &mut Vec<Moto>, eliminados: &mut Vec<Moto>) {
    if let Some(moto) = cola.get(0) {
        println!("Se sacó la siguiente moto de la cola:");
        moto.mostrar();
        eliminados.push(cola.remove(0)); // Mueve la moto a la lista de eliminados
    } else {
        println!("La cola está vacía.");
    }
}

// Función para mostrar las motos en la cola sin mostrar la placa
fn mostrar(cola: &Vec<Moto>) {
    if cola.is_empty() {
        println!("La cola está vacía.");
    } else {
        println!("Motos en la cola:");
        for moto in cola {
            println!("{}", "-".repeat(30)); // Línea de separación
            moto.mostrar();
            println!("{}", "-".repeat(30)); // Línea de separación
        }
    }
}

// Función para mostrar las placas de las motos eliminadas (requiere contraseña)
fn mostrar_placas_eliminadas(eliminados: &Vec<Moto>, contraseña: &str) {
    if contraseña == "admin123" {
        if eliminados.is_empty() {
            println!("La cola está vacía.");
        } else {
            println!("Placas de las motos en la cola:");
            for moto in eliminados {
                println!("ID Cola: {}, Placa: {}", moto.id_cola, moto.placa);
            }
        }
    } else {
        println!("Contraseña incorrecta. No se puede acceder a las placas.");
    }
}

// Función para mostrar las placas de las motos en cola (requiere contraseña)
fn mostrar_placas(cola: &Vec<Moto>, contraseña: &str) {
    if contraseña == "admin123" {
        if cola.is_empty() {
            println!("La cola está vacía.");
        } else {
            println!("Placas de las motos en la cola:");
            for moto in cola {
                println!("ID Cola: {}, Placa: {}", moto.id_cola, moto.placa);
            }
        }
    } else {
        println!("Contraseña incorrecta. No se puede acceder a las placas.");
    }
}

// Función para mostrar la lista de motos eliminadas (requiere contraseña)
fn mostrar_eliminados(eliminados: &Vec<Moto>, contraseña: &str) {
    if contraseña == "admin123" {
        if eliminados.is_empty() {
            println!("No hay motos eliminadas.");
        } else {
            println!("Motos eliminadas:");
            for moto in eliminados {
                println!("{}", "-".repeat(30)); // Línea de separación
                moto.mostrar();
                println!("Placa: {}", moto.placa);
                println!("{}", "-".repeat(30)); // Línea de separación
            }
        }
    } else {
        println!("Contraseña incorrecta. No se puede acceder a las motos eliminadas.");
    }
}

// Funcion para buscar motos en cola
fn buscar_moto_por_id(motos: &Vec<Moto>, id: u32) {
        for moto in motos.iter() {
            if moto.id_cola == id.try_into().unwrap() {
                println!("Moto encontrada:");
                println!("ID: {}", moto.id_cola);
                println!("Placa: {}", moto.placa);
                println!("Marca: {}", moto.marca);
                println!("Cilindraje: {}cc", moto.cilindraje);
                println!("Modelo: {}", moto.modelo);
                println!("Precio: ${}", moto.precio);
                return;
            }
    }
    println!("No se encontró ninguna moto con el ID {}", id);
}

// Funcion para buscar motos en cola eliminada
fn buscar_moto_elim_por_id(motos: &Vec<Moto>, id: u32, contraseña: &str) {
    if contraseña == "admin123" {
                for moto in motos.iter() {
                    if moto.id_cola == id.try_into().unwrap() {
                        println!("Moto encontrada:");
                        println!("ID: {}", moto.id_cola);
                        println!("Placa: {}", moto.placa);
                        println!("Marca: {}", moto.marca);
                        println!("Cilindraje: {}cc", moto.cilindraje);
                        println!("Modelo: {}", moto.modelo);
                        println!("Precio: ${}", moto.precio);
                        return;
                    }
                }
                println!("No se encontró ninguna moto con el ID {}", id);
            
    } else {
        println!("Contraseña incorrecta. No se puede acceder a las motos eliminadas.");
    }
}


// Función para leer una entrada de tipo texto
fn leer_texto(mensaje: &str) -> String {
    let mut entrada = String::new();
    println!("{}", mensaje);
    io::stdin().read_line(&mut entrada).expect("Error al leer entrada");
    entrada.trim().to_string()
}

// Función para leer una entrada de tipo entero
fn leer_entero(mensaje: &str) -> u32 {
    loop {
        let mut entrada = String::new();
        println!("{}", mensaje);
        io::stdin().read_line(&mut entrada).expect("Error al leer entrada");
        match entrada.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Error: Ingrese un número entero válido."),
        }
    }
}

// Función para leer una entrada de tipo flotante
fn leer_flotante(mensaje: &str) -> f32 {
    loop {
        let mut entrada = String::new();
        println!("{}", mensaje);
        io::stdin().read_line(&mut entrada).expect("Error al leer entrada");
        match entrada.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Error: Ingrese un número decimal válido."),
        }
    }
}

// Menú principal
fn menu(cola: &mut Vec<Moto>, eliminados: &mut Vec<Moto>) {
    loop {
        println!("\nMenú:");
        println!("1. Agregar moto");
        println!("2. Sacar moto");
        println!("3. Mostrar todas las motos");
        println!("4. Buscar moto en cola");
        println!("5. Mostrar placas de las motos en cola (requiere contraseña)");
        println!("6. Mostrar motos eliminadas (requiere contraseña)");
        println!("7. Mostrar placas de las motos en cola eliminadas (requiere contraseña)");
        println!("8. Buscar moto en cola eliminada (requiere contraseña)");
        println!("9. Salir");

        let opcion = leer_entero("Ingrese la opción: ");

        match opcion {
            1 => meter(cola),
            2 => sacar(cola, eliminados),
            3 => mostrar(cola),
            4 =>{
                if cola.is_empty() {
                    println!("No hay motos en la cola.");
                } else {
                    let id_busc = leer_entero("Por favor digita el identificador de la moto que quieres buscar el cola");
                    buscar_moto_por_id(&cola, id_busc);
                }
            } 
            5 => {
                let contraseña = leer_texto("Ingrese la contraseña: ");
                mostrar_placas(cola, &contraseña);
            }
            6 => {
                let contraseña = leer_texto("Ingrese la contraseña: ");
                mostrar_eliminados(eliminados, &contraseña);
            }
            7 => {
                let contraseña = leer_texto("Ingrese la contraseña: ");
                mostrar_placas_eliminadas(eliminados, &contraseña);
            }
            8 =>{
                if eliminados.is_empty() {
                    println!("No hay motos eliminadas.");
                } else {
                    let contraseña = leer_texto("Ingrese la contraseña: ");
                    let id_busc = leer_entero("Por favor digita el identificador de la moto que quieres buscar en cola eliminada");
                    buscar_moto_elim_por_id(&eliminados, id_busc, &contraseña);
                }
            } 
            9 => {
                println!("Saliendo del programa.");
                break;
            }
            _ => println!("Opción inválida."),
        }
    }
}

fn main() {
    let mut cola: Vec<Moto> = Vec::new();
    let mut eliminados: Vec<Moto> = Vec::new();
    menu(&mut cola, &mut eliminados);
}
