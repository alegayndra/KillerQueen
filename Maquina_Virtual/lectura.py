from os import DirEntry, read
from pathlib import Path
import io
import sys
from globales import *

def leerCuadruplos(txt_cuadruplos):
  readStr = 0
  while readStr < len(txt_cuadruplos):
    cantidades = txt_cuadruplos[ readStr:txt_cuadruplos.find( '\n', readStr ) ]
    operador = int( cantidades[ 1:cantidades.find( ',' ) ] )
    cantidades = cantidades[ cantidades.find( ',' ) + 2: ]
    varIzq = int(cantidades[ :cantidades.find(',')] )
    cantidades = cantidades[ cantidades.find(',') + 2: ]
    varDer = int(cantidades[ :cantidades.find( ',' ) ] )
    cantidades = cantidades[ cantidades.find(',') + 2:]
    guardar = int(cantidades[ :cantidades.find( ')' ) ] )

    lista_cuadruplos.append((operador, varIzq, varDer, guardar))
    
    readStr = txt_cuadruplos.find('\n', readStr) + 1

def guardarParams(direcciones_params, i):
  readStr = 0
  count = 0
  parm = []
  while readStr < len(direcciones_params) and count < 6:
    cantidades = direcciones_params[readStr:direcciones_params.find('\n', readStr)]
    direccion = int(cantidades[1:cantidades.find(',')] )
    cantidades = cantidades[cantidades.find(',') + 2:]
    tipo = cantidades[:cantidades.find(')')]
    readStr = direcciones_params.find('\n', readStr) + 1
    parm.append((direccion, tipo))
  funciones[i].append(parm)

def guardarFunciones(direcciones_funcs):
  readStr = 0
  i = 0
  while readStr < len(direcciones_funcs):
    funciones.append([])
    cantidades = direcciones_funcs[readStr:direcciones_funcs.find('\n', readStr)]
    nombreFunc = direcciones_funcs[1:cantidades.find(',')]
    cantidades = cantidades[cantidades.find(',') + 2:]

    direcFunc = int(cantidades[:cantidades.find(',')])
    cantidades = cantidades[cantidades.find(',') + 2:]
    numCuadruplo = int(cantidades[:cantidades.find(')')])
    readStr = direcciones_funcs.find('\n', readStr) + 1

    cantidades = direcciones_funcs[readStr:direcciones_funcs.find('\n', readStr)]
    cntIntNormal = int(cantidades[1:cantidades.find(',')])
    cantidades = cantidades[cantidades.find(',') + 2:]
    cntIntTemp = int(cantidades[:cantidades.find(')')])
    readStr = direcciones_funcs.find('\n', readStr) + 1

    cantidades = direcciones_funcs[readStr:direcciones_funcs.find('\n', readStr)]
    readStr = direcciones_funcs.find('\n', readStr) + 1
    cntFloatNormal = int(cantidades[1:cantidades.find(',')])
    cantidades = cantidades[cantidades.find(',') + 2:]
    cntFloatTemp = int(cantidades[:cantidades.find(')')])

    cantidades = direcciones_funcs[readStr:direcciones_funcs.find('\n', readStr)]
    readStr = direcciones_funcs.find('\n', readStr) + 1
    cntCharNormal = int(cantidades[1:cantidades.find(',')])
    cantidades = cantidades[cantidades.find(',') + 2:]
    cntCharTemp = int(cantidades[:cantidades.find(')')])
    funciones[i].append((nombreFunc, direcFunc, numCuadruplo))
    funciones[i].append((cntIntNormal, cntIntTemp))
    funciones[i].append((cntFloatNormal, cntFloatTemp))
    funciones[i].append((cntCharNormal, cntCharTemp))
    guardarParams(direcciones_funcs[direcciones_funcs.find("PARAMS") + 7:direcciones_funcs.find("FIN_PARAMS")], i)
    direcciones_funcs = direcciones_funcs[direcciones_funcs.find("FIN_PARAMS", readStr) + 11:direcciones_funcs.find("FIN_FUNCIONES", readStr)]
    readStr = 0
    i += 1
  
def guardarMapaGlobs(direcciones_globs):
  readStr = 0
  cantidades = direcciones_globs[readStr:direcciones_globs.find('\n', readStr)]
  readStr = direcciones_globs.find('\n', readStr) + 1
  cntIntNormal = cantidades[1:cantidades.find(',')] 
  cantidades = cantidades[cantidades.find(',') + 2:]
  cntIntTemp = cantidades[:cantidades.find(')')]
  
  cantidades = direcciones_globs[readStr:direcciones_globs.find('\n', readStr)]
  readStr = direcciones_globs.find('\n', readStr) + 1
  cntFloatNormal = cantidades[1:cantidades.find(',')] 
  cantidades = cantidades[cantidades.find(',') + 2:]
  cntFloatTemp = cantidades[:cantidades.find(')')]
  
  cantidades = direcciones_globs[readStr:direcciones_globs.find('\n', readStr)]
  readStr = direcciones_globs.find('\n', readStr) + 1
  cntCharNormal = cantidades[1:cantidades.find(',')] 
  cantidades = cantidades[cantidades.find(',') + 2:]
  cntCharTemp = cantidades[:cantidades.find(')')]
  
  mapa_memoria[0][0][0] = [None] * int(cntIntNormal)
  mapa_memoria[0][0][1] = [None] * int(cntIntTemp)
  mapa_memoria[0][1][0] = [None] * int(cntFloatNormal)
  mapa_memoria[0][1][1] = [None] * int(cntFloatTemp)
  mapa_memoria[0][2][0] = [None] * int(cntCharNormal)
  mapa_memoria[0][2][1] = [None] * int(cntCharTemp)

def guardarMapaCons(direcciones_const):
  readStr = 0
  cantidades = direcciones_const[readStr:direcciones_const.find('\n', readStr)]
  cntInt = cantidades[1:cantidades.find(',')]
  cantidades = cantidades[cantidades.find(',') + 2:]
  cntFloat = cantidades[:cantidades.find(',')]
  cantidades = cantidades[cantidades.find(',') + 2:]
  cntChar = cantidades[:cantidades.find(',')]
  mapa_memoria[2][0][0] = [None] * int(cntInt)
  mapa_memoria[2][1][0] = [None] * int(cntFloat)
  mapa_memoria[2][2][0] = [None] * int(cntChar)

  direcciones_const = direcciones_const[direcciones_const.find('\n') + 1:]
  while readStr < len(direcciones_const):
    valYdir = direcciones_const[readStr:direcciones_const.find('\n', readStr)]

    value = valYdir[1:valYdir.find(',')]
    valYdir = valYdir[valYdir.find(',') + 2:]
    direccion = valYdir[:valYdir.find(',')]
    valYdir = valYdir[valYdir.find(',') + 2:]
    tipo = valYdir[:valYdir.find(')')]

    if tipo == "entero":
      value = int(value)
    elif tipo == "flotante":
      value = float(value)
    direccion = int(direccion)
    i = len(dir_memoria[2])-1
    while i >= 0:
      if(direccion >= dir_memoria[2][i][0]):
        mapa_memoria[2][i][0][direccion - dir_memoria[2][i][0]] = value
        break
      i -= 1

    readStr = direcciones_const.find('\n', readStr) + 1

def leer_obj():
  # Lectura y normalizacion de archivo
  prueba_cuadruplos = Path("C:/Users/delca/Documents/Tareas TEC/Compiladores/Compiladores-Proyecto/Compilador/cuadruplos")
  abrir = prueba_cuadruplos / "Killer_queen.txt"
  file_opened = open(abrir, 'r')
  stringTxt = file_opened.read()

  #Registro de valores para constantes en mapa de memoria
  guardarMapaCons(stringTxt[stringTxt.find("CONSTANTES") + 11:stringTxt.find("FIN_CONSTANTES")])

  #Registro de valores para globales en mapa de memoria
  guardarMapaGlobs(stringTxt[stringTxt.find("GLOBALES") + 9:stringTxt.find("FIN_GLOBALES")])

  guardarFunciones(stringTxt[stringTxt.find("FUNCIONES") + 10:stringTxt.find("FIN_FUNCIONES")])
  
  leerCuadruplos(stringTxt[stringTxt.find("CUADRUPLOS") + 11:stringTxt.find("FIN_CUADRUPLOS")])
  '''
  print("Aqui estan las variables globales")
  print(mapa_memoria[0])
  print("")
  print("Aqui estan las variables normales globales")
  print(mapa_memoria[0][0][0], mapa_memoria[0][1][0], mapa_memoria[0][2][0])
  print("")
  print("Aqui estan las variables temporales globales")
  print(mapa_memoria[0][0][1], mapa_memoria[0][1][1], mapa_memoria[0][2][1])
  print("")

  print("Aqui estan las variables locales")
  print(mapa_memoria[1])
  
  print("Aqui estan las variables constantes")
  print(mapa_memoria[2])
  print("")
  print("Aqui estan las variables INT constantes")
  print(mapa_memoria[2][0])
  print("")
  print("Aqui estan las variables FLOAT globales")
  print(mapa_memoria[2][1])
  print("")
  print("Aqui estan las variables CHAR globales")
  print(mapa_memoria[2][2])
  '''