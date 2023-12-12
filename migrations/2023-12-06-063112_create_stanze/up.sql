CREATE TABLE IF NOT EXISTS stanze(
    id_stanza INTEGER NOT NULL PRIMARY KEY NOT NULL,
    id_immobie INTEGER,
    id_tipo_stanza INTEGER,
    mq INTEGER,
    id_agente_edit INTEGER,
    date_update TEXT,
    FOREIGN KEY (id_immobie) REFERENCES immobili(id_immobie),
    FOREIGN KEY (id_agente_edit) REFERENCES agenti (id_agente),
    FOREIGN KEY (id_tipo_stanza) REFERENCES tipi_stanze (id_tipo)
);