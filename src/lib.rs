use anchor_lang::prelude::*;
declare_id!("EZCGbMn2wx2zRfHdd8W9xXFcEV634xhjcpnTB3WGkKxf");

#[program]
pub mod registro_cedulas {

    use super::*;

    pub fn create_reg(context: Context<NuevoReg>, nombre_reg: String) -> Result<()> {

        let owner = context.accounts.owner.key();
        let cedulas = Vec::<Pubkey>::new();

        let reg = Reg{

            owner,
            nombre_reg,
            cedulas

        };
       
        context.accounts.reg.set_inner(reg);

        Ok(())
    }
   


    pub fn create_cedula(context: Context<NuevaCedula>, nombre_reg:String, no_cedula:String, folio: String, curp: String,
                         nombres:String, ap_paterno:String, ap_materno:String, genero: String,
                         institucion:String, profesion: String, entidad:String,
                         ano_registro:String) -> Result<()> {
       
        let con_vida = true;

        let cedula = Cedula { nombre_reg,
                                      no_cedula,
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
                                      con_vida
                                       
        };
       
        context.accounts.cedula.set_inner(cedula);
        context.accounts.reg.cedulas.push(context.accounts.cedula.key());

       
        Ok(())
    }

    pub fn read_cedula(context: Context<ReadCedula>) -> Result<()> {
       
         let cedulas= &context.accounts.reg.cedulas;

        msg!("---------------------------------");
        msg!("{} ", context.accounts.reg.nombre_reg.clone());
        msg!("---------------------------------\n");
        msg!("---------------------------------\n");
   

        let _pos = cedulas

            .iter().position(|&x| x == context.accounts.cedula.key() )
            .ok_or(Errores::CedulaNoExiste)?;    
           

        msg!(

            "{ } { } { }",
            context.accounts.cedula.nombres,
            context.accounts.cedula.ap_paterno,
            context.accounts.cedula.ap_materno,

            );
       
        msg!("No. de C dula: {}", context.accounts.cedula.no_cedula);
        msg!("Folio: {}", context.accounts.cedula.folio);
        msg!("{}",context.accounts.cedula.curp);

        Ok(())
    }


     pub fn update_con_vida(context: Context<UpdateCedula>, _no_cedula : String) -> Result<()> {
       
        let con_vida = context.accounts.cedula.con_vida;
        let nuevo_estado = !con_vida;

        context.accounts.cedula.con_vida = nuevo_estado;

        msg!("La persona asociada a esta c dula {} est  con vida: {}", _no_cedula, nuevo_estado);

        Ok(())
    }


    pub fn delete_cedula(context: Context<DeleteCedula>, _no_cedula: String ) -> Result<()> {


        let cedulas = &mut context.accounts.reg.cedulas ;

        let pos = cedulas

            .iter().position(|&x| x == context.accounts.cedula.key() )
            .ok_or(Errores::CedulaNoExiste)?;    
            cedulas.remove(pos);
         
       
        msg!(
            "La cedula '{}' con el nombre '{}' fue eliminada exitosamente de {}!. Owner id: {}",
            _no_cedula,
            context.accounts.cedula.nombres,
            context.accounts.reg.nombre_reg,
            context.accounts.owner.key()
        );

        Ok(())

    }

}


#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propietario de la cuenta.")]
    NoEresElOwner,

    #[msg("Error, no existe una c dula asociada a este n mero.")]
    CedulaNoExiste,
}

#[account]
#[derive(InitSpace)]
pub struct Reg {
    owner: Pubkey,

    #[max_len(40)]
    nombre_reg: String,

    #[max_len(10)]
    cedulas: Vec<Pubkey>,
}

#[account]
#[derive(InitSpace)]        
pub struct Cedula {

    #[max_len(40)]
    nombre_reg: String,

   #[max_len(9)]
    no_cedula: String,

   #[max_len(10)]
    folio: String,

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

    #[max_len(20)]    
    entidad: String,

   #[max_len(5)]
    ano_registro: String,

    con_vida:bool,

}

#[derive(Accounts)]
pub struct NuevoReg<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Reg::INIT_SPACE+ 8,
        seeds = [b"reg", owner.key().as_ref()],
        bump
    )]
    pub reg: Account<'info, Reg>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction( nombre_reg:String, no_cedula:String)]

pub struct NuevaCedula<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,
     

    #[account(
      init,
      payer = owner,
      space = Cedula::INIT_SPACE + 8,
      seeds = [b"cedula" , no_cedula.as_bytes() , owner.key().as_ref()],
      bump
    )]
    pub cedula : Account<'info, Cedula>,

    #[account(mut)]
    pub reg: Account<'info, Reg>,

   
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct DeleteCedula<'info>{

      #[account(mut)]
      pub owner: Signer<'info>,

      #[account(

        mut,
        close= owner,
        constraint = cedula.nombre_reg == reg.nombre_reg @Errores::CedulaNoExiste

      )]
     
      pub cedula: Account<'info, Cedula>,
     
      #[account(mut)]
      pub reg : Account<'info, Reg>


}

#[derive(Accounts)]
pub struct UpdateCedula<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub cedula: Account<'info, Cedula>,
}

#[derive(Accounts)]
pub struct ReadCedula<'info> {
    pub owner: Signer<'info>,

   
    pub cedula: Account<'info, Cedula>,
    pub reg:Account<'info, Reg>,
}

