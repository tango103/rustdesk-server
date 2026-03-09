# Whitelist de IDs salientes (hbbs)

Este servidor aplica una whitelist **solo para conexiones salientes iniciadas** (A -> B):

- Cualquier cliente puede **recibir** conexiones entrantes.
- Solo IDs autorizados en la whitelist pueden **iniciar** conexión hacia otro ID.

## Ubicación del archivo

Por defecto hbbs carga:

- `whitelist.txt` en el **directorio de trabajo** desde donde ejecutás `hbbs`.

También podés definir la ruta por argumento/env de configuración `whitelist` (según la forma en que levantás hbbs) para apuntar a otro archivo.

## Formato interno de `whitelist.txt`

- Un ID por línea.
- Líneas vacías: se ignoran.
- Líneas que empiezan con `#`: comentario, se ignoran.

Ejemplo:

```txt
# IDs habilitados para iniciar conexiones
123456789
987654321
mi-id-empresa-01
```

## Comportamiento

- Si el ID origen está en whitelist: hbbs permite procesar la solicitud de conexión saliente.
- Si el ID origen no está en whitelist (o no se puede resolver): hbbs rechaza la solicitud saliente.
