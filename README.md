# Registro de cédulas profesionales
![banner](./images/banner-biblioteca.jpg)

CRUD básico de un Solana Program desarrollado con Rust y Anchor desde el Solana Playground. 

Se creó un programa para representar una base de datos de cédulas profesionales. El programa está basado en la poderosa biblioteca-en-solana en donde se modificó el módulo de biblioteca y se le asignó el nombre de registro, representando a la base de datos, y se modificó el módulo de libros como módulo de cédulas, al cual se le añadieron otros datos que representarían a los que se tienen en una cédula profesional los cuales son: número de cédula, folio, curp, nombre, apellidos, género, institución, profesión, entidad, año de registro y se añadiò si la persona está viva o no, esto con el fin de dar de baja una cédula, en caso de que esta haya fallecido. 

Se implementó el programa con validaciones, y también se puede interactuar desde el sceipt en clientes, el cual cuenta con la implementación del método agregarCédula para iniciar el registro.




