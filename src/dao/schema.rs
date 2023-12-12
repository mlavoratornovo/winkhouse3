// @generated automatically by Diesel CLI.

diesel::table! {
    agenti (id_agente) {
        id_agente -> Integer,
        nome -> Text,
        cognome -> Text,
        indirizzo -> Nullable<Text>,
        provincia -> Nullable<Text>,
        cap -> Nullable<Text>,
        citta -> Nullable<Text>,
        username -> Text,
        pass_word -> Text,
        id_agente_edit -> Nullable<Integer>,
        date_update -> Nullable<Text>,
        is_admin -> Nullable<Bool>,
    }
}

diesel::table! {
    anagrafiche (id_anagrafica) {
        id_anagrafica -> Integer,
        cognome -> Text,
        nome -> Nullable<Text>,
        ditta -> Nullable<Bool>,
        indirizzo -> Nullable<Text>,
        provincia -> Nullable<Text>,
        cap -> Nullable<Text>,
        citta -> Nullable<Text>,
        commento -> Nullable<Text>,
        storico -> Nullable<Bool>,
        id_categoria -> Nullable<Integer>,
        codicefiscale -> Nullable<Text>,
        piva -> Nullable<Text>,
        ncivico -> Nullable<Text>,
        id_agente_edit -> Nullable<Integer>,
        date_update -> Nullable<Text>,
    }
}

diesel::table! {
    categorie_clienti (id_categoria) {
        id_categoria -> Integer,
        descrizione -> Text,
        id_agente_edit -> Nullable<Integer>,
        date_update -> Nullable<Text>,
    }
}

diesel::table! {
    classi_energetiche (id_classe) {
        id_classe -> Integer,
        nome -> Text,
        descrizione -> Text,
        id_agente_edit -> Nullable<Integer>,
        date_update -> Nullable<Text>,
    }
}

diesel::table! {
    colloqui (id_colloquio) {
        id_colloquio -> Integer,
        descrizione -> Nullable<Text>,
        id_immobile_abbinato -> Nullable<Integer>,
        tipologia_colloquio -> Nullable<Text>,
        data_ins -> Nullable<Text>,
        data_colloquio -> Nullable<Text>,
        luogo -> Nullable<Text>,
        commento_agenzia -> Nullable<Text>,
        commento_cliente -> Nullable<Text>,
        id_agente_edit -> Nullable<Integer>,
        date_update -> Nullable<Text>,
    }
}

diesel::table! {
    colloqui_agenti (id) {
        id_colloqui_agenti -> Integer,
        id_colloquio -> Nullable<Integer>,
        id_agente -> Nullable<Integer>,
        commento -> Nullable<Text>,
        id_agente_edit -> Nullable<Integer>,
        date_update -> Nullable<Text>,
    }
}

diesel::table! {
    colloqui_anagrafiche (id) {
        id_colloqui_anagrafiche -> Integer,
        id_colloquio -> Nullable<Integer>,
        id_anagrafica -> Nullable<Integer>,
        commento -> Nullable<Text>,
        id_agente_edit -> Nullable<Integer>,
        date_update -> Nullable<Text>,
    }
}

diesel::table! {
    contatti (id_contatto) {
        id_contatto -> Integer,
        contatto -> Text,
        descrizione -> Nullable<Text>,
        id_tipo -> Nullable<Integer>,
        id_anagrafica -> Nullable<Integer>,
        id_agente -> Nullable<Integer>,
        id_agente_edit -> Nullable<Integer>,
        date_update -> Nullable<Text>,
    }
}

diesel::table! {
    immobili (id_immobile) {
        id_immobile -> Integer,
        rif -> Nullable<Text>,
        anno_costruzione -> Nullable<Integer>,
        indirizzo -> Text,
        citta -> Nullable<Text>,
        provincia -> Nullable<Text>,
        cap -> Nullable<Text>,
        zona -> Nullable<Text>,
        data_libero -> Nullable<Text>,
        descrizione -> Nullable<Text>,
        prezzo -> Nullable<Float>,
        mutuo -> Nullable<Float>,
        mutuo_descrizione -> Nullable<Text>,
        spese -> Nullable<Float>,
        varie -> Nullable<Text>,
        visione -> Nullable<Bool>,
        storico -> Nullable<Bool>,
        affitto -> Nullable<Bool>,
        mq -> Nullable<Integer>,
        id_stato -> Nullable<Integer>,
        id_tipologia -> Nullable<Integer>,
        id_riscaldamento -> Nullable<Integer>,
        id_classe_energetica -> Nullable<Integer>,
        ncivico -> Nullable<Text>,
        id_agente_edit -> Nullable<Integer>,
        date_update -> Nullable<Text>,
    }
}

diesel::table! {
    immobili_proprietari (id_immobili_proprietari) {
        id_immobili_proprietari -> Integer,
        id_immobile -> Integer,
        id_anagrafica -> Integer,
    }
}

diesel::table! {
    riscaldamenti (id_riscaldamento) {
        id_riscaldamento -> Integer,
        descrizione -> Text,
        id_agente_edit -> Nullable<Integer>,
        date_update -> Nullable<Text>,
    }
}

diesel::table! {
    stanze (id_stanza) {
        id_stanza -> Integer,
        id_immobile -> Integer,
        id_tipo_stanza -> Nullable<Integer>,
        mq -> Nullable<Integer>,
        id_agente_edit -> Nullable<Integer>,
        date_update -> Nullable<Text>,
    }
}

diesel::table! {
    stati_conservativi (id_stato) {
        id_stato -> Integer,
        descrizione -> Text,
        id_agente_edit -> Nullable<Integer>,
        date_update -> Nullable<Text>,
    }
}

diesel::table! {
    tipi_contatti (id_tipo) {
        id_tipo -> Integer,
        descrizione -> Text,
        id_agente_edit -> Nullable<Integer>,
        date_update -> Nullable<Text>,
    }
}

diesel::table! {
    tipi_immobili (id_tipo) {
        id_tipo -> Integer,
        descrizione -> Text,
        id_agente_edit -> Nullable<Integer>,
        date_update -> Nullable<Text>,
    }
}

diesel::table! {
    tipi_stanze (id_tipo) {
        id_tipo -> Integer,
        descrizione -> Text,
        id_agente_edit -> Nullable<Integer>,
        date_update -> Nullable<Text>,
    }
}

diesel::joinable!(anagrafiche -> agenti (id_agente_edit));
diesel::joinable!(anagrafiche -> categorie_clienti (id_categoria));
diesel::joinable!(categorie_clienti -> agenti (id_agente_edit));
diesel::joinable!(classi_energetiche -> agenti (id_agente_edit));
diesel::joinable!(colloqui -> agenti (id_agente_edit));
diesel::joinable!(colloqui_anagrafiche -> agenti (id_agente_edit));
diesel::joinable!(contatti -> anagrafiche (id_anagrafica));
diesel::joinable!(contatti -> tipi_contatti (id_tipo));
diesel::joinable!(immobili -> agenti (id_agente_edit));
diesel::joinable!(immobili -> classi_energetiche (id_classe_energetica));
diesel::joinable!(immobili -> riscaldamenti (id_riscaldamento));
diesel::joinable!(immobili -> stati_conservativi (id_stato));
diesel::joinable!(immobili_proprietari -> anagrafiche (id_anagrafica));
diesel::joinable!(immobili_proprietari -> immobili (id_immobile));
diesel::joinable!(riscaldamenti -> agenti (id_agente_edit));
diesel::joinable!(stanze -> agenti (id_agente_edit));
diesel::joinable!(stanze -> tipi_stanze (id_tipo_stanza));
diesel::joinable!(stati_conservativi -> agenti (id_agente_edit));
diesel::joinable!(tipi_contatti -> agenti (id_agente_edit));
diesel::joinable!(tipi_immobili -> agenti (id_agente_edit));
diesel::joinable!(tipi_stanze -> agenti (id_agente_edit));

diesel::allow_tables_to_appear_in_same_query!(
    agenti,
    anagrafiche,
    categorie_clienti,
    classi_energetiche,
    colloqui,
    colloqui_agenti,
    colloqui_anagrafiche,
    contatti,
    immobili,
    immobili_proprietari,
    riscaldamenti,
    stanze,
    stati_conservativi,
    tipi_contatti,
    tipi_immobili,
    tipi_stanze,
);
