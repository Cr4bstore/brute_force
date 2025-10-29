main -> always stable, production-ready

develop -> integration of tested features

feature/\_ -> new functionality

fix/\_ -> bugfixes

refactor/\* -> large structural improvements

# Initialize & Protect Base

git init
git add .
git commit -m "initial commit: project structure"
git branch -M main
git checkout -b develop

# Work on features

## Create a new feature branch from develop

git checkout develop
git checkout -b feature/file_io_module

## Work, commit progressively

git add .
git commit -m "Add JSON/CSV file reader module"

## When done:

git push origin feature/file_io_module

## Merge back (after testing)

git checkout develop
git merge --no-ff feature/file_io_module -m "Merge file_io module"
git branch -d feature/file_io_module

# Merge into main (release)

git checkout main
git merge --no-ff develop -m "Release v0.1.0"
git tag -a v0.1.0 -m "First working version"
git push origin main --tags
