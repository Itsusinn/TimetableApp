package cat.itsusinn.timetable.ui.home

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Modifier
import cat.itsusinn.timetable.ffi.add

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun HomeScreen() {
    val showMenu = remember { mutableStateOf(false) }
    val coroutineScope = rememberCoroutineScope()
    val a = 3
    val b = 4
    Scaffold { paddingValues ->
        Column(modifier = Modifier.verticalScroll(rememberScrollState())) {

        }
        Text(text = "$a + $b = ${add(a,b)}", modifier = Modifier.padding(paddingValues))
    }

}