package cat.itsusinn.timetable.ui.home

import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import cat.itsusinn.timetable.ffi.getData
import kotlinx.coroutines.flow.flow

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun HomeScreen() {
    val courses by flow { emit(getData()) }.collectAsState(emptyList())
    Scaffold { padding ->
        LazyColumn(modifier = Modifier.padding(padding)) {
            items(items = courses, itemContent = {item ->
                Row(
                    modifier = Modifier.fillMaxWidth().padding(padding),
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Text(text = "$item")
                }
            })
        }
    }

}
