CREATE TABLE IF NOT EXISTS immobili_proprietari(
    id_immobili_proprietari INTEGER PRIMARY KEY NOT NULL,
    id_immobile INTEGER,
    id_anagrafica INTEGER,
    UNIQUE(id_immobile,id_anagrafica),
    FOREIGN KEY (id_immobile) REFERENCES immobili(id_immobile),
    FOREIGN KEY (id_anagrafica) REFERENCES anagrafiche(id_anagrafica)
);