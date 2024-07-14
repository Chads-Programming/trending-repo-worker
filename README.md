# Trending repo worker
Un cronjob que se activa una vez a la semana y recopila los repositorios trending de github y los envía al 
bot de discord.

## Configuración

### Requisitos
Este proyecto esta realizado con los workers de cloudflare por lo que necesitaras tener encuenta lo siguiente:

- [wrangler](https://developers.cloudflare.com/workers/wrangler/install-and-update/)
- [worker-build](https://crates.io/crates/worker-build)
    - [wasm-pack](https://rustwasm.github.io/wasm-pack/)

### Variables de Entorno
Basarse en el archivo `.dev.vars.example` el cual tiene las siguientes variables de entorno:

- `BOT_ENDPOINT`: Endpoint el cual activará el envío de los repositorios al servidor de discord.
- `BOT_API_KEY`: API key necesaria para acceder al endpoint del bot.
- `API_GITHUB_TOKEN`: Token de acceso de la API de github. 

## Setup
Ejecutar el cronjob en modo desarrollo
```shell
npx wrangler dev --test-scheduled
```

Para probar el cronjob hacer los siguiente:

```shell
curl "http://localhost:8787/__scheduled?cron=0+12+*+*+5"
```

> Nota: el puerto puede ser diferente