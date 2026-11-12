# Prescrire

Rédaction de CCTP et de DPGF pour la maîtrise d'œuvre. Application de bureau pour macOS et Windows.

## Organisation

- `src/` : l'interface (une page web, avec ses polices et ses modules Excel et Word intégrés, sans internet).
- `src-tauri/` : la coquille de bureau (fenêtre, fichiers, dialogues, association des fichiers `.caz`).
- `.github/workflows/fabriquer.yml` : fabrique automatiquement les installateurs Mac et Windows.

## Publier une nouvelle version

1. Augmenter le numéro de version dans `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml` et `package.json`.
2. Envoyer les modifications sur la branche `main` : GitHub fabrique alors le `.dmg` et le `-setup.exe`
   (environ 15 minutes), puis les dépose dans la page **Releases** du dépôt, sous `vX.Y.Z`.

## Bibliothèque partagée

Dans **Bibliothèque > Changer de dossier…**, choisir un dossier du serveur de l'agence. La bibliothèque y est
enregistrée sous `bibliotheque-prescrire.json`. Chaque article porte sa date de modification et son auteur :
quand deux postes écrivent en même temps, les modifications sont fusionnées article par article.

Les projets `.caz` peuvent aussi être rangés sur le serveur. Un fichier caché `.nom.caz.verrou` signale
aux autres postes qu'un projet est ouvert.
