use anchor_lang::prelude::*;

declare_id!("4pKgaXf9xLtqV33sS8MfWYVhDSBbqwFfQH8WZzd4ieH8");

#[program]
pub mod ledger{

    use super::*;

    pub fn create_ledger(context: Context<NuevoLedger>, ledger_nom: String )-> Result<()>{

        let owner = context.accounts.owner.key();

         msg!("Owner id: {}", owner);

         let cedulas: Vec<Cedula> = Vec::new();
        
         context.accounts.ledger.set_inner(Ledger{

                owner,
                ledger_nom,
                cedulas,
                
         });

         Ok(())

    }


    pub fn create_cedula(context: Context<NuevaCedula>, no:u32, fol:u32, curp:String, nom:String,
                      app:String, apm:String, gen:String, inst:String, prof:String, ent:String, ano:u16) ->Result<()>{

                        let cedula: Cedula = Cedula{

                            no_cedula:no,
                            folio:fol,
                            curp,
                            nombres: nom,
                            ap_paterno: app,
                            ap_materno: apm,
                            genero:gen,
                            institucion:inst,
                            profesion:prof,
                            entidad:ent,
                            ano_registro:ano,


                        };

                        context.accounts.ledger.cedulas.push(cedula);

                        Ok(())


    }




    pub fn read_cedulas(context: Context<NuevaCedula>) -> Result<()>{

        
        msg!("Las cedulas son: {:#?}", context.accounts.ledger.cedulas);

        Ok(())



    }


}

#[account]
#[derive(InitSpace)]
pub struct Ledger {

     owner: Pubkey,

     #[max_len(40)]
     ledger_nom: String,
   

     #[max_len(100)]
     cedulas: Vec<Cedula>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Cedula{

        #[max_len(11)]
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

        #[max_len(20)]
        entidad: String,

        #[max_len(5)]
        ano_registro:u16,
}

#[derive(Accounts)]
pub struct NuevoLedger<'info>{

        #[account(mut)]
        pub owner: Signer<'info>,

        #[account(
            init,
            payer = owner,
            space = Ledger::INIT_SPACE + 8,
            seeds = [b"ledger", owner.key().as_ref()],
            bump
        )]
        pub ledger:Account<'info, Ledger>,
        pub system_program: Program<'info, System>, 

}

#[derive(Accounts)]
#[instruction(ledger_nom:String)]
pub struct NuevaCedula<'info> {

        pub owner: Signer<'info>,
        

        pub ledger:Account<'info, Ledger>,
        
}
