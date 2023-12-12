CREATE TABLE IF NOT EXISTS contatti(
    id_contatto INTEGER PRIMARY KEY NOT NULL,
    contatto TEXT NOT NULL,
    descrizione TEXT,
    id_tipo INTEGER,
    id_anagrafica INTEGER,
    id_agente INTEGER,
    id_agente_edit INTEGER,
    date_update TEXT,
    FOREIGN KEY (id_agente_edit) REFERENCES agenti (id_agente),
    FOREIGN KEY (id_agente) REFERENCES agenti (id_agente),
    FOREIGN KEY (id_anagrafica) REFERENCES anagrafiche (id_anagrafica),
    FOREIGN KEY (id_tipo) REFERENCES tipi_contatti (id_tipo)
)

