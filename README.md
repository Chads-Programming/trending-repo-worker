> Todavia esta construccion

# Trending repo worker
Un cronjob que se activa una vez a la semana y recopila los repositorios trending de github y los envía al 
bot de discord

## Setup
Ejecutar el cronjob en modo desarrollo
```shell
npx wrangler dev --test-scheduled
```

Para probar el cronjob hacer los siguiente:

```shell
curl "http://localhost:8787/__scheduled?cron=0+12+*+*+5"
```