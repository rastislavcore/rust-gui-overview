#include "Bindings.h"

#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQuickStyle>

int main(int argc, char *argv[])
{
    Q_INIT_RESOURCE(qml); // in order to use qrc: resources, when linking staticaly: https://doc.qt.io/qt-5/resources.html
    QQuickStyle::setStyle("Material"); //
    QGuiApplication app(argc, argv);
    qmlRegisterType<Todos>("RustCode", 1, 0, "Todos");

    QQmlApplicationEngine engine;
    engine.load(QUrl(QStringLiteral("qrc:/main.qml")));
    if (engine.rootObjects().isEmpty())
        return -1;

    return app.exec();
}
