#ifndef MINIPORT_H
#define MINIPORT_H

typedef long NTSTATUS;

#define STATUS_SUCCESS 0L

NTSTATUS DriverEntry(void *driver_object, void *registry_path);

#endif // MINIPORT_H
