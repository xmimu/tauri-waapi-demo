import { computed, ref, type Ref } from "vue";

export function usePaginatedTable<T>(
  source: Ref<T[]>,
  initialPageSize = 20,
) {
  const pageSize = ref(initialPageSize);
  const currentPage = ref(1);

  const totalCount = computed(() => source.value.length);
  const paginatedList = computed(() => {
    const start = (currentPage.value - 1) * pageSize.value;
    return source.value.slice(start, start + pageSize.value);
  });

  function onPageChange(page: number) {
    currentPage.value = page;
  }

  function onSizeChange(size: number) {
    pageSize.value = size;
    currentPage.value = 1;
  }

  function resetPage() {
    currentPage.value = 1;
  }

  return {
    pageSize,
    currentPage,
    totalCount,
    paginatedList,
    onPageChange,
    onSizeChange,
    resetPage,
  };
}
