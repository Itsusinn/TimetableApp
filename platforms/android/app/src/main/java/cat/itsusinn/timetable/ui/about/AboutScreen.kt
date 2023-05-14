package cat.itsusinn.timetable.ui.about

import androidx.compose.foundation.layout.padding
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
fun AboutScreen() {
    val showMenu = remember { mutableStateOf(false) }
    val coroutineScope = rememberCoroutineScope()
    Scaffold{ paddingValues ->
        Text(text = "about", modifier = Modifier.padding(paddingValues))
    }
}