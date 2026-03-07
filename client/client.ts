////////Imports ////////////////////
import { PublicKey } from "@solana/web3.js";

////////////////// Constantes ////////////////////
const n_registro = "Registro de Cédulas Profesionales"; // Nombre de la biblioteca
const owner = pg.wallet.publicKey; // Wallet

//////////////////// Client Test Logs ////////////////////
console.log("My address:", owner.toString()); // Ver el adress
const balance = await pg.connection.getBalance(owner);
console.log(`My balance: ${balance / web3.LAMPORTS_PER_SOL} SOL`); // Ver el la cantidad de tokens de solana

//////////////////// FUNCIONES ////////////////////

//////////////////// OBTENER PDAs ////////////////////
/*
Un PDA representa una cuenta que es controlada por un programa (smart contract), y una de sus principales caracteristicas es no contar
con una clave privada con la cual firmar al momento de realizar alguna transaccion (transferencia, escritura o modificacion de un dato)
dentro del contrato. En su lugar, emplea direcciones generadas deterministicamente, es decir, recreables a partir de semillas.
Las semillas pueden ser varias y de diferentes tipos, puede depender desde un valor predefenidio (como es usualmente el valor de la semilla 1),
hasta de direcciones secundarias (como la del caller u otra cuenta).

Es por ello que para llamar desde el front una funcion del Solana Program desplegado es necesario contar con las semillas en su orden y tipo
correspondiente. Se recomienda no usar valores sencillos (que no solo dependan de valores predefinidos), pero tampoco se encuentren
compuestas de valores redundantes (como el program id o alguna cuenta padre).
*/
//////////////////// Biblioteca ////////////////////
function pdaRegistro(n_registro) {
  console.log(n_registro);
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("registro"), // Semilla 1: b"biblioteca"
      // Buffer.from(n_biblioteca), // Semilla 2: nombre de la biblioteca  -> String
      owner.toBuffer(), // Semilla 3: wallet -> Pubkey
    ],
    pg.PROGRAM_ID // Program ID: Siempre va al final
  );
}
//////////////////// Libro ////////////////////
function pdaCedula(n_cedula) {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("cedula"), // Semilla 1: b"libro"
      Buffer.from(n_cedula), // Semilla 2: nombre del libro: -> String
      owner.toBuffer(), // Semilla 3: wallet -> Pubkey
    ],
    pg.PROGRAM_ID // Program ID: Siempre va al final
  );
}

//////////////////// Crear Biblioteca ////////////////////
// Para crear la biblioteca solo es necesario el nombre que tendra
async function createRegistro(n_registro) {
  const [pda_registro] = pdaRegistro(n_registro); // Primero se obtiene la cuenta de la biblioteca

  const txHash = await pg.program.methods // mediante la libreria pg (solana playground) se acceden a los metodos del programa
    .createRegistro(n_registro) // crear biblioteca
    .accounts({
      // Se agregan las cuentas de las que depende (Contexto del struct NuevaBiblioteca)
      owner: owner,
      registro: pda_registro,
    })
    .rpc();

  console.log("txHash: ", txHash);
}

//////////////////// Agregar Libro ////////////////////
// Para crear un libro solo es necesario pasar el libro y el numero de paginas. El estado se define automaticamente en el programa
async function createCedula(n_registro, n_cedula, folio, curp, nombres, apPaterno, apMaterno, genero, institucion, profesion, entidad, anoRegistro) {
  // Agregar Libro
  const [pda_cedula] = pdaCedula(n_cedula); // se determina la cuenta del libro
  const [pda_registro] = pdaRegistro(n_registro); // se obtiene la cuenta de la biblioteca

  const txHash = await pg.program.methods
    .createCedula(n_registro, n_cedula, folio, curp, nombres, apPaterno, apMaterno, genero, institucion, profesion, entidad, anoRegistro ) // agregar_libro
    .accounts({
      // cuentas del contexto
      owner: owner,
      cedula: pda_cedula,
      registro: pda_registro,
    })
    .rpc();

  console.log("txHash: ", txHash);
}

//////////////////// Alternar estado ////////////////////
// Para cambiar el estado de true a false o visceversa solo se necesita el nombre del libro
async function updateConVida(n_cedula) {
  // Modificar Libro
  const [pda_cedula] = pdaCedula(n_cedula); // se determina la cuenta del libro
  const [pda_registro] = pdaRegistro(n_registro); // se obtiene la cuenta de la biblioteca

  const txHash = await pg.program.methods
    .updateConVida(n_cedula) // alternar_estado
    .accounts({
      // cuentas del contexto
      owner: owner,
      cedula : pda_cedula,
    })
    .rpc();

  console.log("txHash: ", txHash);
}

//////////////////// Eliminar Libro ////////////////////
// Para eliminar un libro solo es necesario proporcionar el nombre del libro a eliminar de la biblioteca
async function deleteCedula(n_cedula) {
  // Eliminar Libro
  const [pda_cedula] = pdaCedula(n_cedula); // se determina la cuenta del libro
  const [pda_registro] = pdaRegistro(n_registro); // se obtiene la cuenta de la biblioteca
  const txHash = await pg.program.methods
    .deleteCedula(n_cedula) // eliminar_libro
    .accounts({
      // cuentas del contexto
      owner: owner,
      cedula: pdaCedula,
      registro: pda_registro,
    })
    .rpc();

  console.log("txHash: ", txHash);
}

//////////////////// Ver Libros ////////////////////
/*
 Anteriormente, en la version anterior de la biblioteca, esta instruccion se encotraba implementada dentro del Solana Program, pero... ¿porque ya no?
 En la prinmera version de la biblioteca los libros eran structs contenidos en un vector dentro de la cuenta biblioteca. Al ser elementos de un vector
 su visualizacion era mas simple. En este caso, cada libro se encuentra definido por una cuenta, por lo que visualizar informacion de multiples cuentas
 desde el Solana Program es ineficiente a comparacion de hacerlo desde el frontend.

Para lograr hacerlo es necesario realizar los siguientes pasos:

1. Determinar el PDA de la biblioteca
2. Obtener el vector de libros (direcciones)
3. Por cada direccion, obtener la informacion del libro
4. Mostrarla con console.log
*/
async function readCedula(n_registro) {
  // Ver Libros
  const [pda_registro] = pdaRegistro(n_registro); // se obtiene la cuenta de la biblioteca

  try {
    // Se accede a los datos de la cuenta (biblioteca)
    const registroAccount = await pg.program.account.registro.fetch(
      pda_registro
    );

    // Mediante el .length se obtiene el tamaño del vector de libros en laa biblioteca
    const numero_cedulas = registroAccount.cedulas.length;

    // Se verifican si hay libros en el vector
    if (!registroAccount.cedulas || numero_cedulas === 0) {
      console.log("El registro se encuentra vacío");
      return;
    }

    // Se imprime el valor en la consola
    console.log("Cantidad de libros:", numero_cedulas);

    // Se itera cada cuenta (libro) del vector (biblioteca) y se obtiene la informacion asociada
    for (let i = 0; i < numero_cedulas; i++) {
      const cedulaKey = registroAccount.cedulas[i];

      const cedulaAccount = await pg.program.account.cedula.fetch(cedulaKey);
      console.log(cedulaAccount);
      // Finaliza mostrando en la terminal la informacion de cada libro
      console.log(`${n_registro}\n
        ${cedulaAccount.nombres}" "${cedulaAccount.apPaterno}" "${cedulaAccount.apMaterno} \n
        * No. de cédula: ${cedulaAccount.noCedula} \n * Folio: ${cedulaAccount.folio} \n
        * CURP: ${cedulaAccount.curp} \n * Género: ${cedulaAccount.genero} \n * Institución:
        ${cedulaAccount.institucion} \n * Entidad: ${cedulaAccount.entidad} \n * Año de registro:
        ${cedulaAccount.anoRegistro} \n * Con Vida: ${cedulaAccount.conVida}
        \n * Dirección(PDA): ${cedulaKey.toBase58()}`);

    }
  } catch (error) {
    console.error("Error viendo cedulas:", error);

    // Debugging adicional
    if (error.message) {
      console.error("Mensaje de error:", error.message);
    }
    if (error.logs) {
      console.error("Logs del programa:", error.logs);
    }
  }
}

//createRegistro(n_registro);

createCedula(n_registro, "20125482", "100", "FASD800734lkujnh12", "Jorge", "Gutierrez", "Roldán", "Masc",
             "Universidad Galaxia", "Lic. en Enfermería", "Colima", "2005");
// eliminarLibro("El alquimista");
// cambiarEstado("El alquimista 2");
//readCedula(n_registro);

// solana confirm -v <txHash>


