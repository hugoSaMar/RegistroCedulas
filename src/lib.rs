use anchor_lang::prelude::*;
declare_id!("Bf83QEa2253iwmKj3uRFETjoMuyN7CRRnetVa4uwyL5J");

#[program]
pub mod registro {
    use super::*;

    pub fn create_registro(context: Context<NuevoRegistroCed>, nombre_reg: String) -> Result<()> {
        let owner = context.accounts.owner.key();
        let cedulas: Vec<Cedula> = Vec::new();

        context.accounts.registro.set_inner(RegistroCed {
            owner,
            nombre_reg,
            cedulas,
        });

        Ok(())
    }

    pub fn create_cedula(context: Context<NuevaCedula>, no_cedula:u32, folio: u32, curp: String,
                         nombres:String, ap_paterno:String, ap_materno:String, genero: String,
                         institucion:String, profesion: String, entidad:String, 
                         ano_registro:u16) -> Result<()> {
        
        
        let cedula: Cedula = Cedula { no_cedula, 
                                      folio, 
                                      curp, 
                                      nombres, 
                                      ap_paterno, 
                                      ap_materno, 
                                      genero, 
                                      institucion, 
                                      profesion,
                                      entidad,
                                      ano_registro, 
                                        
        }; 

        context.accounts.registro.cedulas.push(cedula);

        Ok(())
    }

    pub fn read_cedula(context: Context<NuevaCedula>) -> Result<()> {
        msg!(
            "La lista de cedulas es: {:#?}",
            context.accounts.registro.cedulas
        );

        Ok(())
    }

  /*   pub fn eliminar_libro(context: Context<NuevoLibro>, nombre: String) -> Result<()> {
        let libros = &mut context.accounts.biblioteca.libros;

        for libro in 0..libros.len() {
            if libros[libro].nombre == nombre {
                libros.remove(libro);
                msg!("Libro {nombre} eliminado!");
                return Ok(());
            }
        }

        Err(Errores::LibroNoExiste.into())
    } */

    /*  pub fn alternar_estado(context: Context<NuevoLibro>, nombre: String) -> Result<()> {
        let libros = &mut context.accounts.biblioteca.libros;

        for libro in 0..libros.len() {
            let estado = libros[libro].disponible;

            if libros[libro].nombre == nombre {
                let nuevo_estado = !estado;
                libros[libro].disponible = nuevo_estado;

                msg!(
                    "El libro: {} ahora tiene un valor de disponibilidad: {}",
                    nombre,
                    nuevo_estado
                );
                return Ok(());
            }
        }

        Err(Errores::CedulaNoExiste.into())
    }*/
}

#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propietario de la cuenta.")]
    NoEresElOwner,

    #[msg("Error, no existe una cédula asociada a este número.")]
    CedulaNoExiste,
}

#[account]
#[derive(InitSpace)]
pub struct RegistroCed {
    owner: Pubkey,

    #[max_len(60)]
    nombre_reg: String,

    #[max_len(100)]
    cedulas: Vec<Cedula>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Cedula {
    
    #[max_len(12)]
    no_cedula: u32,

    #[max_len(12)]
    folio: u32,

    #[max_len(18)]
    curp: String,

    #[max_len(50)]
    nombres: String,

    #[max_len(20)]
    ap_paterno: String,

    #[max_len(20)]
    ap_materno: String,

    #[max_len(20)]
    genero: String,

    #[max_len(50)]
    institucion: String,

    #[max_len(30)]
    profesion: String,

    #[max_len(60)]    
    entidad: String,

    #[max_len(60)]
    ano_registro: u16,

}

#[derive(Accounts)]
pub struct NuevoRegistroCed <'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = RegistroCed::INIT_SPACE + 8,
        seeds = [b"registro", owner.key().as_ref()],
        bump
    )]
    pub registro: Account<'info, RegistroCed>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NuevaCedula<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub registro: Account<'info, RegistroCed>,
}
