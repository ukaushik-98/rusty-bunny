# Rust Bunny!
Start using Rust Bunny with:
- Import Rust Bunny into browser
- Make it the default engine
- Type `rb` to trigger!

Bookmark manager that supports:
- Github: `gh`
- Twitter: `tw`
- Google: 

## Development
- Build via `cargo install --path .`
- Run `cargo run`
- Testing `cargo test`

## Usage 
### Add to FireFox
To set up the new application to be used as a custom search engine in Firefox, you can follow these steps:

- Install the “Add custom search engine” Firefox Add-on
- Open up the extension
- Fill out the form with the following values:
  - Name: Rusty Bunny Local (you can use whatever you want here)
  - Search URL: http://localhost:8000/search?cmd=%s
- Click “Add custom search engine”
- Check the box “Make this the current search engine”
- Click “Add”

### Add to Chrome
- Navigate to chrome://settings/searchEngines
- Click “Add” under “Default Search Engines” and use the following values:
- Search Engine: Rusty Bunny Local
- Keyword: rb (triggers the search engine, if this search engine is not the default)
- URL: http://localhost:8000/search?cmd=%s
- Under “Other search engines”, find your search engine, select the 3 dots menu and select “Make default”