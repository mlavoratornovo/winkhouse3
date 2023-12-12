use diesel::prelude::*;

#[diesel(check_for_backend(diesel::sqlite::Sqlite))]

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Agenti, foreign_key = id_agente_edit))]
#[diesel(table_name = crate::schema::tipi_contatti)]
pub struct TipiContatti{
    id_tipo:i32,
    descrizione:String,
    id_agente_edit:Options<i32>,
    date_update:Options<Date>
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Agenti, foreign_key = id_agente))]
#[diesel(belongs_to(Agenti, foreign_key = id_agente_edit))]
#[diesel(belongs_to(Anagrafiche, foreign_key = id_anagrafica))]
#[diesel(belongs_to(TipiContatti, foreign_key = id_tipo))]
#[diesel(table_name = crate::schema::contatti)]
pub struct Contatti{
    id_contatto:i32,
    contatto:String,
    descrizione:Option<String>,
    id_tipo:Options<i32>,
    id_anagrafica:Options<i32>,
    id_agente:Options<i32>,
    id_agente_edit:Options<i32>,
    date_update:Options<Date>
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Agenti, foreign_key = id_agente_edit))]
#[diesel(table_name = crate::schema::agenti)]
pub struct Agenti{
    id_agente:i32,
    nome:String,
    cognome:String,
    indirizzo:Option<String>,
    provincia:Option<String>,
    cap:Option<String>,
    citta:Option<String>,
    username:Option<String>,
    pass_word:Option<String>,
    id_agente_edit: Option<i32>,
    date_update:Option<Date>,
    is_admin:bool    
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Agenti, foreign_key = id_agente_edit))]
#[diesel(table_name = crate::schema::categorie_clienti)]
pub struct CategorieClienti{
    id_categoria:i32,
    descrizione:String,
    id_agente_edit: Option<i32>,
    date_update:Option<Date>
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(belongs_to(Agenti, foreign_key = id_agente_edit))]
#[diesel(belongs_to(CategorieClienti, foreign_key = id_categoria))]
#[diesel(table_name = crate::schema::anagrafiche)]
pub struct Anagrafiche {
    id_anagrafica:i32,
    cognome:String,
    nome:Option<String>,
    ditta:bool,
    indirizzo:Option<String>,
    provincia:Option<String>,
    cap:Option<String>,
    citta:Option<String>,
    commento:Option<String>,
    storico:bool,
    id_categoria:Option<i32>,
    codicefiscale:Option<String>,
    piva:Option<String>,
    ncivico:Option<String>,
    id_agente_edit: Option<i32>,
    date_update:Option<Date>
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Agenti, foreign_key = id_agente_edit))]
#[diesel(table_name = crate::schema::classi_energetiche)]
pub struct ClassiEnergetiche{
    id_categoria:i32,
    descrizione:String,
    id_agente_edit: Option<i32>,
    date_update:Option<Date>
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Agenti, foreign_key = id_agente_edit))]
#[diesel(table_name = crate::schema::stati_conservativi)]
pub struct StatiConservativi{
    id_stato:i32,
    descrizione:String,
    id_agente_edit: Option<i32>,
    date_update:Option<Date>
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Agenti, foreign_key = id_agente_edit))]
#[diesel(table_name = crate::schema::stati_conservativi)]
pub struct Riscaldamenti{
    id_riscaldamento:i32,
    descrizione:String,
    id_agente_edit:Option<i32>,
    date_update:Option<Date>
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Agenti, foreign_key = id_agente_edit))]
#[diesel(table_name = crate::schema::tipi_immobili)]
pub struct TipiImmobili{
    id_tipo:i32,
    descrizione:String,
    id_agente_edit:Option<i32>,
    date_update:Option<Date>
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Agenti, foreign_key = id_agente_edit))]
#[diesel(belongs_to(ClassiEnergetiche, foreign_key = id_classe_energetica))]
#[diesel(belongs_to(StatiConservativi, foreign_key = id_stato))]
#[diesel(belongs_to(Riscaldamenti, foreign_key = id_riscaldamento))]
#[diesel(belongs_to(TipiImmobili, foreign_key = id_tipologia))]
#[diesel(table_name = crate::schema::immobili)]
pub struct Immobili{    
    id_immobile:i32,
    rif:Option<String>,
    anno_costruzione:Option<i32>,
    indirizzo:String,
    citta:Option<String>,
    provincia:Option<String>,
    cap:Option<String>,
    zona:Option<String>,
    data_libero:Option<Date>,
    descrizione:Option<String>,
    prezzo:Option<Float>,
    mutuo:Option<Float>,
    mutuo_descrizione:Option<String>,
    spese:Option<Float>,
    varie:Option<Float>,
    visione:bool,
    storico:bool,
    affitto:bool,
    mq:Option<i32>,
    id_stato:Option<i32>,
    id_tipologia:Option<i32>,
    id_riscaldamento:Option<i32>,
    id_classe_energetica:Option<i32>,
    ncivico:Option<String>,
    id_agente_edit: Option<i32>,
    date_update:Option<Date>
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Agenti, foreign_key = id_agente_edit))]
#[diesel(table_name = crate::schema::tipi_stanze)]
pub struct TipiStanze{
    id_tipo:i32,
    descrizione:String,
    id_agente_edit:Option<i32>,
    date_update:Option<Date>
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Agenti, foreign_key = id_agente_edit))]
#[diesel(belongs_to(Immobili, foreign_key = id_immobile))]
#[diesel(belongs_to(TipiStanze, foreign_key = id_tipo_stanza))]
#[diesel(table_name = crate::schema::stanze)]
pub struct Stanze{
    id_stanza:i32,
    id_immobile:i32,
    id_tipo_stanza:Option<i32>,
    mq:Option<i32>,
    id_agente_edit:Option<i32>,
    date_update:Option<Date>
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Immobili, foreign_key = id_immobile))]
#[diesel(belongs_to(Anagrafiche, foreign_key = id_anagrafica))]
#[diesel(table_name = crate::schema::immobili_proprietari)]
pub struct ImmobiliProprietari{
    id_immobili_proprietari:i32,
    id_immobile:i32,
    id_anagrafica:i32
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Immobili, foreign_key = id_immobile_abbinato))]
#[diesel(belongs_to(Agenti, foreign_key = id_agente_edit))]
#[diesel(table_name = crate::schema::colloqui)]
pub struct Colloqui{
    id_colloquio:i32,
    descrizione:Option<String>,
    id_immobile_abbinato:Option<i32>,
    tipologia_colloquio:Option<String>,
    data_ins:Option<Date>,
    data_colloquio:Option<Date>,
    luogo:Option<String>,
    commento_agenzia:Option<String>,
    commento_cliente:Option<String>,
    id_agente_edit:Option<i32>,
    date_update:Option<Date>
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Agenti, foreign_key = id_agente_edit))]
#[diesel(belongs_to(Agenti, foreign_key = id_agente))]
#[diesel(belongs_to(Colloqui, foreign_key = id_colloquio))]
#[diesel(table_name = crate::schema::colloqui_agenti)]
pub struct ColloquiAgenti{
    id_colloqui_agenti:i32,
    id_colloquio:Option<i32>,
    id_agente:Option<i32>,
    commento:Option<String>,
    id_agente_edit:Option<i32>,
    date_update:Option<Date>
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Agenti, foreign_key = id_agente_edit))]
#[diesel(belongs_to(Anagrafiche, foreign_key = id_anagrafica))]
#[diesel(belongs_to(Colloqui, foreign_key = id_colloquio))]
#[diesel(table_name = crate::schema::colloqui_anagrafiche)]
pub struct ColloquiAnagrafiche{
    id_colloqui_anagrafiche:i32,
    id_colloquio:Option<i32>,
    id_anagrafica:Option<i32>,
    commento:Option<String>,
    id_agente_edit:Option<i32>,
    date_update:Option<Date>
}