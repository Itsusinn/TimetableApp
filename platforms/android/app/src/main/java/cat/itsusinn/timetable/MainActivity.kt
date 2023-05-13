package cat.itsusinn.timetable

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Column
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import cat.itsusinn.timetable.ffi.add

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            Column {
                MessageCard("Android")
                addNum(a = 3, b = 4)
            }
        }
    }
}

@Composable
fun MessageCard(name: String) {
    Text(text = "Hello $name!")
}

@Composable
fun addNum(a:Int,b:Int) {
    Text(text = "$a + $b = ${add(a,b)}")
}