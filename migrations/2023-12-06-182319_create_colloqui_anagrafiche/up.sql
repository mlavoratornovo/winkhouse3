CREATE TABLE IF NOT EXISTS colloqui_anagrafiche(
    id_colloqui_anagrafiche INTEGER PRIMARY KEY NOT NULL,
    id_colloquio INTEGER,
    id_anagrafica INTEGER,
    commento TEXT,
    id_agente_edit INTEGER,
    date_update TEXT,
    FOREIGN KEY (id_agente_edit) REFERENCES agenti (id_agente),
    FOREIGN KEY (id_anagrafica) REFERENCES anagrafica (id_anagrafica)
);
