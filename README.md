# Dias

Scaffold new projects from templates.

```bash
USAGE:
    dias [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --templates <templates>    Sets a custom templates directory.
```

## Templates

A project template consists of a `template/` directory and a `manifest.yml` file (see [examples](./example)), both within their own directory.

Template files should use the [Handlebars](https://handlebarsjs.com/) format and be under the `template/` directory.  The directory structure of the template files will be maintained in the scaffolded project.

The manifest requires the field `project_type` - this value will be used when selecting the project to scaffold.

Any variables used in the project's templates should be listed under `variables` in `manifest.yml`.

## Templates Location

`dias` looks for templates directly within its immediate directory.  If you need to specify a custom location for templates then use the `--templates` option.
