CREATE TABLE IF NOT EXISTS colloqui(
    id_colloquio INTEGER NOT NULL PRIMARY KEY NOT NULL,
    descrizione TEXT,
    id_immobile_abbinato INTEGER,
    tipologia_colloquio TEXT,
    data_ins TEXT,
    data_colloquio TEXT,
    luogo TEXT,
    commento_agenzia TEXT,
    commento_cliente TEXT,
    id_agente_edit INTEGER,
    date_update TEXT,
    FOREIGN KEY (id_agente_edit) REFERENCES agenti (id_agente),
    FOREIGN KEY (id_immobile_abbinato) REFERENCES immobili(id_immobile)
);
