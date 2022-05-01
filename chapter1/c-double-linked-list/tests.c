#include "CuTest.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include "dllists.h"

// Finding things at the beginning, end and middle of a list
void TestInsertFind(CuTest *tc) {
    Element *head = NULL;  
    head = insert(head, "Planet");
    head = insert(head, "Earth");
    head = insert(head, "Mars");
    head = insert(head, "Moon");

    Element *a1 = find(head, "Planet");
    CuAssertPtrNotNull(tc, a1);

    Element *a2 = find(head, "Earth");
    CuAssertPtrNotNull(tc, a2);

    Element *a3 = find(head, "Moon");
    CuAssertPtrNotNull(tc, a3);

    Element *no = find(head, "Jupiter");
    CuAssertPtrEquals(tc, NULL, no);
}

// Empty list tests
void TestFindEmpty(CuTest *tc) {
    Element *head = NULL;  
    Element *no = find(head, "Jupiter");
    CuAssertPtrEquals(tc, NULL, no);
}

// Test print list
void TestPrintList(CuTest *tc) {
    Element *head = NULL;  
    head = insert(head, "Vietnam");
    head = insert(head, "Morning");
    head = insert(head, "Good");

    char *actual = format_to_string(head);
    CuAssertStrEquals(tc, "Good, Morning, Vietnam", actual);

    free(actual);
}

// Test list creation and deletion
void TestCreateDelete(CuTest *tc) {
    Element *head = NULL;  

    char *a1 = format_to_string(head);
    CuAssertStrEquals(tc, "", a1);
    free(a1);

    head = insert(head, "a");

    char *a2 = format_to_string(head);
    CuAssertStrEquals(tc, "a", a2);
    free(a2);

    // Delete only element
    head = remove_if_found(head, "a");

    char *a3 = format_to_string(head);
    CuAssertStrEquals(tc, "", a3);
    free(a3);

    // Remove middle element
    head = insert(head, "a");
    head = insert(head, "b");
    head = insert(head, "c");

    head = remove_if_found(head, "b");

    char *a4 = format_to_string(head);
    CuAssertStrEquals(tc, "c, a", a4);
    free(a4);

    // Remove last element
    head = remove_if_found(head, "a");

    char *a5 = format_to_string(head);
    CuAssertStrEquals(tc, "c", a5);
    free(a5);

    // Remove first element leaving remainder
    head = insert(head, "d");

    head = remove_if_found(head, "d");

    char *a6 = format_to_string(head);
    CuAssertStrEquals(tc, "c", a6);
    free(a6);
}

CuSuite* DLListSuite() {
    CuSuite* suite = CuSuiteNew();
    SUITE_ADD_TEST(suite, TestInsertFind);
    SUITE_ADD_TEST(suite, TestFindEmpty);
    SUITE_ADD_TEST(suite, TestPrintList);
    SUITE_ADD_TEST(suite, TestCreateDelete);
    return suite;
}

void RunAllTests(void) {
    CuString *output = CuStringNew();
    CuSuite* suite = CuSuiteNew();
    
    CuSuiteAddSuite(suite, DLListSuite());

    CuSuiteRun(suite);
    CuSuiteSummary(suite, output);
    CuSuiteDetails(suite, output);
    printf("%s\n", output->buffer);
}

int main(void) {
    RunAllTests();
}
