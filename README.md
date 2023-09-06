# Video Count by Channel

## Dataset

Usa el dataset de [estadisticas de youtube](https://www.kaggle.com/datasets/datasnaek/youtube-new). Los archivos `.csv` se deben extraer en una carpeta llamada `data` en la raiz del proyecto

## Implementaciones

Hay tres implementaciones:

- sequential: Utiliza un solo hilo de ejecución
- concurrent: Utiliza un hilo de ejecución por cada linea de cada archivo.
- semi-concurrent: Utiliza un hilo de ejecución por cada archivo, pero las lineas de una archivo se procesan secuencialmente.

La implementación semi-concurrente es mas rapida (por alguna razón que desconozco).
