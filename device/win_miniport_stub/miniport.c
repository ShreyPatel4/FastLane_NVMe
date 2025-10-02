#include "miniport.h"

NTSTATUS DriverEntry(void *driver_object, void *registry_path) {
    (void)driver_object;
    (void)registry_path;
    return STATUS_SUCCESS;
}
