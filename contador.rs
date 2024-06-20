use anchor_lang::prelude::*;

declare_id!("BNvUNcc7xZxcocBGHjAVxqR5rSecJaVACoD9QDhsvC5b"); // Siempre se debe agregar un id al programa

#[program]
mod programa_contador {
    use super::*; // Le estamos diciendo que puede tener acceso a todo lo que está por fuera

    // Instrucciones
    pub fn crear_contador(ctx: Context<Crear>, primer_numero: u64) -> Result<()> { // Result: Devuelve que todo está bien o que todo falló
        ctx.accounts.contador.numero = primer_numero;
        ctx.accounts.contador.autoridad = ctx.accounts.autoridad.key();
        msg!("Creando un contador con número inicial {}", primer_numero);
        Ok(()) // Siempre será última línea de una instrucción en anchor
    }

    pub fn borrar_contador(ctx: Context<Borrar>) -> Result<()> {
        msg!("Contador eliminado");
        Ok(())
    }
}

// Lista de cuentas con las que va a trabajar la instrucción creada, es el contexto de una instrucción
#[derive(Accounts)] // Con este macro se crea un contexto de una instrucción
pub struct Crear<'info> { // 'info: es un lifetime, se le puede poner cualquier nombre
    // space = 8 bytes para el discriminador + lo que ocupe tu estructura
    #[account(init, payer = autoridad, space = 8 + 8 + 32)] // Le indica a contador que es una estructura account y le da ciertos atributos.
    pub contador: Account<'info, Contador>,

    #[account(mut)] // mut: Indica que va a cambiar
    pub autoridad: Signer<'info>, // La autoridad es variable porque tiene que pagar la renta de la cuenta contador

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Borrar<'info> {
    #[account(mut)]
    pub autoridad: Signer<'info>,

    #[account(
        mut,
        constraint = contador.autoridad == contador.key(), // Estamos preguntando si el contador.autoridad es el mismo que firmó al hacer el borrar
        close = autoridad
    )]
    pub contador: Account<'info, Contador>
}

#[account]
pub struct Contador {
    numero: u64, // 8 bytes
    autoridad: Pubkey, // 32 bytes
}
