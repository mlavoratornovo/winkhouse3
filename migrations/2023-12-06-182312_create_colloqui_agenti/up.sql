CREATE TABLE IF NOT EXISTS colloqui_agenti(
    id_colloqui_agenti INTEGER PRIMARY KEY NOT NULL,
    id_colloquio INTEGER,
    id_agente INTEGER,
    commento TEXT,
    id_agente_edit INTEGER,
    date_update TEXT,
    FOREIGN KEY (id_agente_edit) REFERENCES agenti (id_agente),
    FOREIGN KEY (id_agente) REFERENCES agenti (id_agente)
);
