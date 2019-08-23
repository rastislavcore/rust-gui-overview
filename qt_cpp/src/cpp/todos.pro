HEADERS       = Bindings.h
SOURCES       = main.cpp \
                Bindings.cpp
 RESOURCES    = qml.qrc

QT += widgets qml quick quickcontrols2
requires(qtConfig(combobox))


LIBS += -L../../release -ltodos
