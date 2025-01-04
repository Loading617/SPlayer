#include <gtk/gtk.h>
#include <gst/gst.h>

int main(int argc, char *argv[]) {
    GtkWidget *window;
    
    gtk_init(&argc, &argv);

    gst_init(&argc, &argv);

    window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_title(GTK_WINDOW(window), "CPlayer");
    gtk_window_set_default_size(GTK_WINDOW(window), 400, 300);

    gtk_widget_show_all(window);

    gtk_main();

    return 0;
}
